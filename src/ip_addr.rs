use serde::{Deserialize, Serialize};

/// IpAddr : Contains an IP adresse.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpAddr {
    /// String identifying a IPv4 address formatted in the 'dotted decimal' notation as defined in RFC 1166.
    #[serde(rename = "ipv4Addr", skip_serializing_if = "Option::is_none")]
    pub ipv4_addr: Option<String>,
    #[serde(rename = "ipv6Addr", skip_serializing_if = "Option::is_none")]
    pub ipv6_addr: Option<Box<crate::Ipv6Addr>>,
    #[serde(rename = "ipv6Prefix", skip_serializing_if = "Option::is_none")]
    pub ipv6_prefix: Option<Box<crate::Ipv6Prefix>>,
}

impl IpAddr {
    /// Contains an IP adresse.
    pub fn new() -> IpAddr {
        IpAddr {
            ipv4_addr: None,
            ipv6_addr: None,
            ipv6_prefix: None,
        }
    }
}
