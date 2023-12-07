use serde::{Deserialize, Serialize};

/// AddrFqdn : IP address and/or FQDN.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddrFqdn {
    #[serde(
        rename = "ipAddr",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ip_addr: Option<Option<Box<crate::IpAddr>>>,
    /// Indicates an FQDN.
    #[serde(rename = "fqdn", skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
}

impl AddrFqdn {
    /// IP address and/or FQDN.
    pub fn new() -> AddrFqdn {
        AddrFqdn {
            ip_addr: None,
            fqdn: None,
        }
    }
}
