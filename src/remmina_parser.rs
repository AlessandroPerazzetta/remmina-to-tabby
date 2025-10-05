use std::fs;
use std::path::PathBuf;

use std::io::{BufRead, BufReader};
// use crate::remmina_types::{RemminaProfile,RemminaFiles};
use crate::remmina_types::{RemminaFiles, RemminaProfile, SshAuthMethod};
use crate::protocols_types::ALLOWED_PROTOCOLS_EXPORT;

#[allow(dead_code)]
/// Methods for RemminaFiles
impl RemminaFiles {
    /// Find all .remmina files in the given directory
    /// 
    /// # Arguments
    /// 
    /// * `remmina_dir` - A string slice that holds the path to the Remmina directory
    /// 
    /// # Returns
    /// 
    /// * `Result<Self, std::io::Error>` - A RemminaFiles struct or an IO error
    /// 
    /// # Errors
    /// 
    /// Returns an error if the directory cannot be read or accessed
    pub fn find(remmina_dir: &str) -> Result<Self, std::io::Error> {
        let entries = fs::read_dir(remmina_dir)?;
        
        let files: Vec<PathBuf> = entries
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| {
                path.is_file() && 
                path.extension()
                    .and_then(|ext| ext.to_str())
                    == Some("remmina")
            })
            .collect();
        
        Ok(RemminaFiles { files })
    }

    /// Find all .remmina files in the given directory (fallback version)
    /// Returns empty collection on any error for backward compatibility
    /// 
    /// # Arguments
    /// 
    /// * `remmina_dir` - A string slice that holds the path to the Remmina directory
    /// 
    /// # Returns
    /// 
    /// * `Self` - A RemminaFiles struct, empty if directory cannot be read
    pub fn find_safe(remmina_dir: &str) -> Self {
        Self::find(remmina_dir).unwrap_or_else(|_| RemminaFiles { 
            files: Vec::new() 
        })
    }

    /// Show all found .remmina files
    pub fn show_files(&self) {
        for path in &self.files {
            println!("Found remmina file: {}", path.display());
        }
    }

    /// Check if file contains a line starting with "protocol=" and show the value
    /// 
    /// # Behavior
    /// * If protocol is "SSH", print available
    /// * If protocol is "RDP" or "VNC", print not implemented
    /// * If protocol is unrecognized, print warning protocol not recognized
    pub fn check_protocols(&self) {
        for path in &self.files {
            let mut found = false;
            if let Ok(file) = fs::File::open(path) {
                let reader = BufReader::new(file);
                for line_result in reader.lines() {
                    match line_result {
                        Ok(line) => {
                            if let Some(rest) = line.strip_prefix("protocol=") {
                                let protocol = rest.trim().to_uppercase();
                                match protocol.as_str() {
                                    "SSH" => {
                                        println!("{}: protocol={} ✅ [available]", path.display(), protocol);
                                    }
                                    "RDP" | "VNC" => {
                                        println!("{}: protocol={} ❌ [not implemented]", path.display(), protocol);
                                    }
                                    _ => {
                                        println!("{}: protocol not recognized ({}) ⚠️", path.display(), protocol);
                                    }
                                }
                                found = true;
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!("Warning: Error reading line from {}: {}", path.display(), e);
                            break; // Stop processing this file on error
                        }
                    }
                }
            }
            if !found {
                println!("{}: protocol not found ❌", path.display());
            }
        }
    }

    /// Return a new RemminaFiles containing only files with any of the given protocols (case-insensitive)
    /// 
    /// # Arguments
    /// * `protocols` - A slice of strings representing the protocols to filter by
    /// # Returns
    /// * `RemminaFiles` - A new RemminaFiles struct containing only the filtered files
    /// # Behavior
    /// * For each file, read lines and look for "protocol=" line
    /// * If the protocol matches any in the given list (case-insensitive), include the file in the result
    /// * If no match, exclude the file
    /// * If file cannot be read, skip it
    pub fn filter_by_protocols(&self, protocols: &[String]) -> RemminaFiles {
        let mut filtered_files = Vec::new();

        for path in &self.files {
            if let Ok(file) = fs::File::open(path) {
                let reader = BufReader::new(file);
                for line_result in reader.lines() {
                    match line_result {
                        Ok(line) => {
                            if let Some(rest) = line.strip_prefix("protocol=") {
                                let proto = rest.trim().to_uppercase();
                                if protocols.iter().any(|p| p == &proto) {
                                    filtered_files.push(path.clone());
                                }
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!("Warning: Error reading line from {}: {}", path.display(), e);
                            break; // Stop processing this file on error
                        }
                    }
                }
            }
        }

        RemminaFiles {
            files: filtered_files,
        }
    }

    /// Show files that use protocols in ALLOWED_PROTOCOLS_EXPORT
    /// 
    /// # Arguments
    /// * `execute` - If true, perform the export (currently just prints a message
    /// * If false, just print what would be done (dry-run)
    /// # Behavior
    /// * For each file, read lines and look for "protocol=" line
    /// * If the protocol is in ALLOWED_PROTOCOLS_EXPORT, print the file path and protocol
    /// * If `execute` is true, print "Exporting" message
    /// * If `execute` is false, print "Dry-run" message
    /// * If file cannot be read, skip it
    pub fn export_profiles_base(&self, execute: bool) {
        for path in &self.files {
            if let Ok(file) = fs::File::open(path) {
                let reader = BufReader::new(file);
                for line_result in reader.lines() {
                    match line_result {
                        Ok(line) => {
                            if let Some(rest) = line.strip_prefix("protocol=") {
                                let protocol = rest.trim().to_uppercase();
                                if ALLOWED_PROTOCOLS_EXPORT.iter().any(|&p| p == protocol) {
                                    if execute {
                                        println!(" ⬅️  Exporting: {} (protocol={}) ✅", path.display(), protocol);
                                    } else {
                                        println!("Dry-run: {} (protocol={})", path.display(), protocol);
                                    }
                                }
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!("Warning: Error reading line from {}: {}", path.display(), e);
                            break; // Stop processing this file on error
                        }
                    }
                }
            }
        }
    }

    /// Extract profiles with name, server, group, protocol from files using ALLOWED_PROTOCOLS_EXPORT
    /// 
    /// # Returns
    /// * `Vec<RemminaProfile>` - A vector of RemminaProfile structs containing extracted profile information
    /// # Behavior
    /// * For each file, read lines and look for "name=", "server=", "group=", "protocol=" lines
    /// * If the protocol is in ALLOWED_PROTOCOLS_EXPORT, create a RemminaProfile struct with the extracted information
    /// * Add the RemminaProfile to the result vector
    /// * If file cannot be read, skip it
    pub fn export_profiles(&self) -> Vec<RemminaProfile> {
        let mut profiles = Vec::new();

        for path in &self.files {
            let mut name = None;
            let mut server = None;
            let mut port = None;
            let mut group = None;
            let mut protocol = None;
            let mut user = None;
            let mut auth_method = None;
            let mut ssh_auth_value = None;
            let mut rdp_auth_value = None; // Prototype for RDP auth
            let mut vnc_auth_value = None; // Prototype for VNC auth

            // First pass: collect all key-value pairs
            if let Ok(file) = fs::File::open(path) {
                let reader = BufReader::new(file);
                for line_result in reader.lines() {
                    match line_result {
                        Ok(line) => {
                            let line = line.trim();
                            if let Some(rest) = line.strip_prefix("name=") {
                                name = Some(rest.to_string());
                            } else if let Some(rest) = line.strip_prefix("server=") {
                                server = Some(rest.to_string());
                            } else if let Some(rest) = line.strip_prefix("user=") {
                                if !rest.is_empty() {
                                    user = Some(rest.to_string());
                                }
                            } else if let Some(rest) = line.strip_prefix("group=") {
                                group = Some(rest.to_string());
                            } else if let Some(rest) = line.strip_prefix("protocol=") {
                                let proto = rest.to_uppercase();
                                if ALLOWED_PROTOCOLS_EXPORT.iter().any(|&p| p == proto) {
                                    protocol = Some(proto);
                                }
                            } else if let Some(rest) = line.strip_prefix("ssh_auth=") {
                                ssh_auth_value = Some(rest.to_string());
                            } else if let Some(rest) = line.strip_prefix("rdp_auth=") {
                                rdp_auth_value = Some(rest.to_string()); // Store for future use
                            } else if let Some(rest) = line.strip_prefix("vnc_auth=") {
                                vnc_auth_value = Some(rest.to_string()); // Store for future use
                            } else if let Some(rest) = line.strip_prefix("port=") {
                                port = Some(rest.to_string());
                            }
                        }
                        Err(e) => {
                            eprintln!("Warning: Error reading line from {}: {}", path.display(), e);
                            break;
                        }
                    }
                }
            }

            // Now, after protocol is known, handle ssh_auth if protocol is SSH
            if let (Some(ref proto), Some(ref rest)) = (protocol.as_ref(), ssh_auth_value.as_ref()) {
                if proto.as_str() == "SSH" {
                    let method = rest.parse::<u8>()
                        .map(crate::remmina_types::get_auth_method_from_int)
                        .unwrap_or_else(|_| SshAuthMethod::from_str(rest));
                    if let SshAuthMethod::Unknown(ref s) = method {
                        eprintln!("Warning: Unknown SSH auth method '{}' in file {}", s, path.display());
                    }
                    auth_method = Some(method);
                }
            }

            // Prototype: Handle RDP auth (future implementation)
            if let (Some(ref proto), Some(ref rest)) = (protocol.as_ref(), rdp_auth_value.as_ref()) {
                if proto.as_str() == "RDP" {
                    // TODO: Implement RDP auth method parsing
                    println!("(Prototype) Found RDP auth method '{}' in file {}", rest, path.display());
                    // Example: auth_method = Some(RdpAuthMethod::from_str(rest));
                }
            }

            // Prototype: Handle VNC auth (future implementation)
            if let (Some(ref proto), Some(ref rest)) = (protocol.as_ref(), vnc_auth_value.as_ref()) {
                if proto.as_str() == "VNC" {
                    // TODO: Implement VNC auth method parsing
                    println!("(Prototype) Found VNC auth method '{}' in file {}", rest, path.display());
                    // Example: auth_method = Some(VncAuthMethod::from_str(rest));
                }
            }

            if protocol.is_some() {
                let profile = RemminaProfile {
                    name,
                    server,
                    port,
                    group,
                    protocol,
                    user,
                    path: path.clone(),
                };

                println!(" ⬅️  Exporting Profile:");
                println!("    • Name:     {}", profile.name.as_deref().unwrap_or("<none>"));
                println!("    • Server:   {}", profile.server.as_deref().unwrap_or("<none>"));
                println!("    • Port:     {}", profile.port.as_deref().unwrap_or("<none>"));
                println!("    • User:     {}", profile.user.as_deref().unwrap_or("<none>"));
                println!("    • Group:    {}", profile.group.as_deref().unwrap_or("<none>"));
                println!("    • Protocol: {}", profile.protocol.as_deref().unwrap_or("<none>"));
                println!("    • Auth Method: {}", auth_method.as_ref().map(|m| format!("{:?}", m)).unwrap_or_else(|| "<none>".to_string()));
                println!("    • Path:     {}", profile.path.display());

                profiles.push(profile);
            }
        }

        profiles
    }
}
