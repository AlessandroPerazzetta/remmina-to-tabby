// use std::{path::PathBuf, u8};
use std::path::PathBuf;

/// Struct to hold a list of .remmina files
pub struct RemminaFiles {
    pub files: Vec<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct RemminaProfile {
    pub name: Option<String>,
    pub server: Option<String>,
    pub port: Option<String>,
    pub group: Option<String>,
    pub protocol: Option<String>,
    pub user: Option<String>,

    pub path: std::path::PathBuf,
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SshAuthMethod {
    Password,
    SSHIdentityFile,
    SSHAgent,
    PublicKey,
    KerberosGSSAPI,
    KerberosInteractive,
    Unknown(String),
}

impl SshAuthMethod {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "password" => SshAuthMethod::Password,
            "sshidentityfile" => SshAuthMethod::SSHIdentityFile,
            "sshagent" => SshAuthMethod::SSHAgent,
            "publickey" => SshAuthMethod::PublicKey,
            "kerberosgssapi" => SshAuthMethod::KerberosGSSAPI,
            "kerberosinteractive" => SshAuthMethod::KerberosInteractive,
            other => SshAuthMethod::Unknown(other.to_string()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SshAuthMethod::Password => "password",
            SshAuthMethod::SSHIdentityFile => "sshidentityfile",
            SshAuthMethod::SSHAgent => "sshagent",
            SshAuthMethod::PublicKey => "publickey",
            SshAuthMethod::KerberosGSSAPI => "kerberosgssapi",
            SshAuthMethod::KerberosInteractive => "kerberosinteractive",
            SshAuthMethod::Unknown(s) => s.as_str(),
        }
    }
}

/// Get authentication method as u8
/// 0 = password
/// 1 = sshidentityfile
/// 2 = sshagent
/// 3 = publickey
/// 4 = kerberosgssapi
/// 5 = kerberosinteractive
/// Any other value = unknown
/// 
/// # Arguments
/// * `method` - A reference to an SshAuthMethod enum
/// # Returns
/// * `u8` - The corresponding u8 value for the authentication method
/// # Examples
/// ```
/// let method = SshAuthMethod::Password;
/// let method_u8 = get_auth_method(&method);
/// assert_eq!(method_u8, 0);
/// ```
pub fn get_auth_method_as_int(method: &SshAuthMethod) -> u8 {
    match method {
        SshAuthMethod::Password => 0,
        SshAuthMethod::SSHIdentityFile => 1,
        SshAuthMethod::SSHAgent => 2,
        SshAuthMethod::PublicKey => 3,
        SshAuthMethod::KerberosGSSAPI => 4,
        SshAuthMethod::KerberosInteractive => 5,
        SshAuthMethod::Unknown(_) => u8::MAX, // unknown
    }
}
/// Get authentication method from u8
/// 0 = password
/// 1 = sshidentityfile
/// 2 = sshagent
/// 3 = publickey
/// 4 = kerberosgssapi
/// 5 = kerberosinteractive
/// Any other value = unknown
///
/// # Arguments
/// * `method` - A u8 value representing the authentication method
/// # Returns
/// * `SshAuthMethod` - The corresponding SshAuthMethod enum variant
/// # Examples
/// ```
/// let method_u8 = 0;
/// let method = get_auth_method_from_u8(method_u8);
/// assert_eq!(method, SshAuthMethod::Password);
/// ```
pub fn get_auth_method_from_int(method: u8) -> SshAuthMethod {
    match method {
        0 => SshAuthMethod::Password,
        1 => SshAuthMethod::SSHIdentityFile,
        2 => SshAuthMethod::SSHAgent,
        3 => SshAuthMethod::PublicKey,
        4 => SshAuthMethod::KerberosGSSAPI,
        5 => SshAuthMethod::KerberosInteractive,
        _ => SshAuthMethod::Unknown("unknown".to_string()),
    }
}