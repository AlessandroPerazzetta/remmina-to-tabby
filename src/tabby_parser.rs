// use std::path::{Path, PathBuf};
use std::path::Path;
use std::fs;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use serde_yaml_ng::Value;

use crate::remmina_types::RemminaProfile;
use crate::protocols_types::{ConnectionProtocols, get_default_port_for_protocol};

#[derive(Default, Debug, Serialize, Deserialize)]
/// Represents the options for a profile, with various optional fields.
pub struct ProfileOptions {
    pub host: Option<String>,
    pub user: Option<String>,
    pub algorithms: Option<serde_yaml_ng::Value>,
    pub input: Option<serde_yaml_ng::Value>,
    pub auth: Option<String>,
    pub port: u16,
    // Add other fields as needed
}

#[derive(Debug, Serialize, Deserialize)]
/// Represents a profile with various attributes and options.
pub struct Profile {
    #[serde(default = "Profile::default_type")]
    pub r#type: String,
    pub name: String,
    #[serde(default = "Profile::default_icon")]
    pub icon: String,
    pub options: ProfileOptions, //Option<serde_yaml_ng::Value>,
    #[serde(default = "Profile::default_weight")]
    pub weight: i32,
    #[serde(default = "Profile::default_color")]
    pub color: String,
    pub group: Option<String>,
    pub id: Option<String>,
    // Add other fields as needed
}
#[derive(Debug, Serialize, Deserialize)]
/// Represents a group with an ID and name.
pub struct Group {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Represents the entire configuration structure.
pub struct TabbyConfig {
    pub version: u32,
    pub profiles: Vec<Profile>,
    pub groups: Option<Vec<Group>>,
    // Add other fields as needed
    
    /// This field captures any extra fields from the YAML that are not explicitly defined
    /// in the TabbyConfig struct. It allows deserialization of unknown or future fields
    /// without breaking, storing them in a BTreeMap with their names and values.
    #[serde(flatten)]
    extra: std::collections::BTreeMap<String, Value>,

}

/// Provide default values for Profile fields
impl Default for Profile {
    fn default() -> Self {
        Profile {
            r#type: Profile::default_type(),
            name: String::new(),
            icon: Profile::default_icon(),
            options: ProfileOptions::default(),
            weight: Profile::default_weight(),
            color: Profile::default_color(),
            group: None,
            id: None,
        }
    }
}

/// Default values for Profile fields
impl Profile {
    fn default_type() -> String {
        "ssh".to_string()
    }
    fn default_icon() -> String {
        "fas fa-terminal".to_string()
    }
    fn default_weight() -> i32 {
        -1
    }
    fn default_color() -> String {
        "#FF9C00".to_string() // or any default you want
    }
}

/// Methods for TabbyConfig
impl TabbyConfig {
    /// Try to load config.yaml from the given directory and parse it into TabbyConfig
    /// 
    /// # Arguments
    /// * `dir` - The directory where config.yaml is located.
    /// # Returns
    /// * `Result<Self, String>` - Ok(TabbyConfig) if successful, Err(String) with error message if failed.
    /// # Errors
    /// * If the config.yaml file does not exist in the specified directory.
    /// * If there is an error reading the file.
    /// * If there is an error parsing the YAML content.
    /// # Example
    /// ```
    /// let config = TabbyConfig::load_from_dir("/path/to/tabby/dir")?;
    /// ```
    /// # Notes
    /// This function uses the `serde_yaml_ng` crate for YAML parsing.
    pub fn load_from_dir(dir: &str) -> Result<Self, String> {
        let config_path = Path::new(dir).join("config.yaml");
        if !config_path.exists() {
            return Err(format!("Error: config.yaml not found in directory '{dir}'"));
        }
        let content = fs::read_to_string(&config_path)
            .map_err(|e| format!("Error reading config.yaml: {e}"))?;
        let config: TabbyConfig = serde_yaml_ng::from_str(&content)
            .map_err(|e| format!("Error parsing config.yaml: {e}"))?;
        Ok(config)
    }


    /// Returns a reference to the profile with the given name, if it exists.
    /// 
    /// # Arguments
    /// * `name` - The name of the profile to search for.
    ///
    /// # Returns
    /// * `Option<&Profile>` - Some reference to the profile if found, or None if not found.
    pub fn get_profile(&self, name: &str) -> Option<&Profile> {
        self.profiles.iter().find(|p| p.name == name)
    }
    
    /// Adds a new profile to the profiles list.
    ///
    /// # Arguments
    /// * `profile` - The Profile to add.
    pub fn add_profile(&mut self, profile: Profile) {
        self.profiles.push(profile);
    }

    /// Generates a unique profile ID in the format: [protocol]:[identifier]:[name]:[uuid]
    /// Notes: generate_profile_uuid is an instance method to ensure uniqueness within the TabbyConfig
    /// as an instance of TabbyConfig to check if in config a profile with same id exists, else regenerate new uuid
    /// 
    /// # Arguments
    /// * `protocol` - The protocol type (e.g., "ssh").
    /// * `identifier` - A custom identifier (e.g., "custom").
    /// * `name` - The profile name
    /// 
    /// # Returns
    /// * `String` - The generated unique profile ID.
    /// 
    pub fn generate_profile_uuid(&self, protocol: &str, identifier: &str, name: &str) -> String {
        loop {
            let uuid = Uuid::new_v4();
            let profile_id = format!("{protocol}:{identifier}:{name}:{uuid}");
            if !self.profiles.iter().any(|p| p.id.as_deref() == Some(&profile_id)) {
                return profile_id;
            }
        }
    }

    /// Generates a random group ID (UUID v4) as a string.
    ///
    /// # Returns
    /// * `String` - The generated group ID.
    pub fn generate_group_id() -> String {
        Uuid::new_v4().to_string()
    }

    /// Creates a new group with the given name, generates a unique ID, and adds it to the groups list.
    /// If the groups list is None, it initializes it with the new group.
    /// 
    /// # Arguments
    /// * `name` - The name of the group to add.
    /// # Returns
    /// * `String` - The ID of the newly created group, or the existing group's ID if it already exists.
    /// 
    /// Notes:
    ///     Currently (01/10/2025) Remmina support sub groups, a sub group is identified with "/" in the name
    ///     Tabby does not support sub groups, multiple git requests as reference:
    ///         - https://github.com/Eugeny/tabby/issues/9210
    ///         - https://github.com/Eugeny/tabby/issues/6522
    ///         - https://github.com/Eugeny/tabby/issues/6408
    ///         - https://github.com/Eugeny/tabby/issues/411
    ///   
    ///     So if a group with "/" is found, it will be created as is
    ///     and it will be up to the user to manage it in Tabby
    ///     Future improvement could be to create a flat group structure in Tabby by replacing "/" with " - " or similar
    ///     but for now we keep it simple and create groups as is
    pub fn add_group(&mut self, name: &str) -> String {
        // Check if group with the same name already exists
        if let Some(groups) = &mut self.groups {
            if let Some(existing_group) = groups.iter().find(|g| g.name == name) {
                println!(" └── Group '{name}' already exists.");
                existing_group.id.clone()
            } else {
                let new_group = Group {
                    id: TabbyConfig::generate_group_id(),
                    name: name.to_string(),
                };
                let group_id = new_group.id.clone();
                groups.push(new_group);
                group_id
            }
        } else {
            let new_group = Group {
                id: TabbyConfig::generate_group_id(),
                name: name.to_string(),
            };
            let group_id = new_group.id.clone();
            self.groups = Some(vec![new_group]);
            group_id
        }
    }
    
    /// Imports multiple profiles into the TabbyConfig.
    /// 
    /// # Arguments
    /// * `profiles` - A vector of Profile instances to be added.
    ///
    pub fn import_profiles(&mut self, profiles: Vec<RemminaProfile >) -> usize {
        let mut imported_count = 0;
        for profile in profiles {
            // println!("➡️ Importing profile: {:?}", profile);
            println!(" ➡️  Importing Profile: '{}' (protocol={})", profile.name.clone().unwrap_or_default(), profile.protocol.clone().unwrap_or_default());

            if self.get_profile(&profile.name.clone().unwrap_or_default()).is_some() {
                println!(" └── Profile '{}' already exists. Skipping import.", profile.name.clone().unwrap_or_default());
                continue;
            } else {
                let profile_id = self.generate_profile_uuid(
                    &profile.protocol.clone().unwrap_or_default().to_lowercase(),
                    "custom",
                    &profile.name.clone().unwrap_or_default(),
                );
                println!(" └── Generated profile UUID: {profile_id}");

                let group_id = self.add_group(profile.group.as_deref().unwrap_or("Default Group"));
                // println!(" └── Using group id: {:?} - name: {:?}", group_id, profile.group);
                println!(" └── Using group id: {:?} - name: {:?}", group_id, profile.group.as_deref().unwrap_or("Default Group"));

                // println!("Profile port: {:?}", profile.port);


                // Set default port based on protocol if port is None
                let proto = ConnectionProtocols::from_str(profile.protocol.as_deref().unwrap_or(""));
                let port = profile.port
                    .as_ref()
                    .and_then(|p| p.parse::<u16>().ok())
                    .unwrap_or_else(|| get_default_port_for_protocol(&proto));

                if profile.port.is_some() {
                    println!(" └── Remmina profile port: {:?}", profile.port);
                } else {
                    println!(" └── Remmina profile port not set, using default for protocol [{:?}]: {}", proto.as_str(), get_default_port_for_protocol(&proto));
                }

                let new_profile_options = ProfileOptions {
                    host: profile.server.clone(),
                    user: profile.user.clone(),
                    algorithms: Some(serde_yaml_ng::Value::Mapping(Default::default())),
                    input: Some(serde_yaml_ng::Value::Mapping(Default::default())),
                    auth: Some("password".to_string()),
                    port,
                };
                let new_profile = Profile {
                    r#type: profile.protocol.clone().unwrap_or_default().to_lowercase(),
                    name: profile.name.clone().unwrap_or_default(),
                    options: new_profile_options,
                    group: Some(group_id),
                    id: Some(profile_id),
                    ..Default::default()
                };
                self.add_profile(new_profile);
            }
            imported_count += 1;
        }
        imported_count
    }

    /// Saves the TabbyConfig as YAML to the given path.
    ///
    /// # Arguments
    /// * `path` - The file path where the YAML should be saved.
    /// # Returns
    /// * `Result<(), String>` - Ok if successful, Err with error message if failed.
    pub fn save_to_path(&self, path: &str) -> Result<(), String> {
        let yaml = serde_yaml_ng::to_string(self)
            .map_err(|e| format!("Error serializing config to YAML: {e}"))?;
        std::fs::write(path, yaml)
            .map_err(|e| format!("Error writing YAML to file: {e}"))?;
        Ok(())
    }


}