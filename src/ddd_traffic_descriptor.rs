use serde::{Deserialize, Serialize};

/// DddTrafficDescriptor : Contains a Traffic Descriptor.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DddTrafficDescriptor {
    /// String identifying a IPv4 address formatted in the 'dotted decimal' notation as defined in RFC 1166.
    #[serde(rename = "ipv4Addr", skip_serializing_if = "Option::is_none")]
    pub ipv4_addr: Option<String>,
    #[serde(rename = "ipv6Addr", skip_serializing_if = "Option::is_none")]
    pub ipv6_addr: Option<Box<crate::Ipv6Addr>>,
    /// Unsigned Integer, i.e. only value 0 and integers above 0 are permissible.
    #[serde(rename = "portNumber", skip_serializing_if = "Option::is_none")]
    pub port_number: Option<i32>,
    /// String identifying a MAC address formatted in the hexadecimal notation according to clause 1.1 and clause 2.1 of RFC 7042.
    #[serde(rename = "macAddr", skip_serializing_if = "Option::is_none")]
    pub mac_addr: Option<String>,
}

impl DddTrafficDescriptor {
    /// Contains a Traffic Descriptor.
    pub fn new() -> DddTrafficDescriptor {
        DddTrafficDescriptor {
            ipv4_addr: None,
            ipv6_addr: None,
            port_number: None,
            mac_addr: None,
        }
    }
}
