/// List of allowed protocols
/// Notes:
///     Currently (01/10/2025) only "SSH" is supported in Tabby, no reason to allow other protocols
///     Multiple git requests as reference:
///         - https://github.com/Eugeny/tabby/issues/6918
///         - https://github.com/Eugeny/tabby/issues/6411
///         - https://github.com/Eugeny/tabby/issues/6408
///         - https://github.com/Eugeny/tabby/issues/5854
pub const ALLOWED_PROTOCOLS_EXPORT: &[&str] = &["SSH"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionProtocols {
    Ssh,
    Rdp,
    Vnc,
    Unknown(String),
}

impl ConnectionProtocols {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "ssh" => ConnectionProtocols::Ssh,
            "rdp" => ConnectionProtocols::Rdp,
            "vnc" => ConnectionProtocols::Vnc,
            other => ConnectionProtocols::Unknown(other.to_string()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ConnectionProtocols::Ssh => "ssh",
            ConnectionProtocols::Rdp => "rdp",
            ConnectionProtocols::Vnc => "vnc",
            ConnectionProtocols::Unknown(s) => s.as_str(),
        }
    }
}

pub fn get_default_port_for_protocol(protocol: &ConnectionProtocols) -> u16 {
    match protocol {
        ConnectionProtocols::Ssh => 22,
        ConnectionProtocols::Rdp => 3389,
        ConnectionProtocols::Vnc => 5900,
        ConnectionProtocols::Unknown(_) => 0,
    }
}