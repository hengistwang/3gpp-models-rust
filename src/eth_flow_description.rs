use serde::{Deserialize, Serialize};

/// EthFlowDescription : Identifies an Ethernet flow.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EthFlowDescription {
    /// String identifying a MAC address formatted in the hexadecimal notation according to clause 1.1 and clause 2.1 of RFC 7042.
    #[serde(rename = "destMacAddr", skip_serializing_if = "Option::is_none")]
    pub dest_mac_addr: Option<String>,
    #[serde(rename = "ethType")]
    pub eth_type: String,
    /// Defines a packet filter of an IP flow.
    #[serde(rename = "fDesc", skip_serializing_if = "Option::is_none")]
    pub f_desc: Option<String>,
    #[serde(rename = "fDir", skip_serializing_if = "Option::is_none")]
    pub f_dir: Option<Box<crate::FlowDirection>>,
    /// String identifying a MAC address formatted in the hexadecimal notation according to clause 1.1 and clause 2.1 of RFC 7042.
    #[serde(rename = "sourceMacAddr", skip_serializing_if = "Option::is_none")]
    pub source_mac_addr: Option<String>,
    #[serde(rename = "vlanTags", skip_serializing_if = "Option::is_none")]
    pub vlan_tags: Option<Vec<String>>,
    /// String identifying a MAC address formatted in the hexadecimal notation according to clause 1.1 and clause 2.1 of RFC 7042.
    #[serde(rename = "srcMacAddrEnd", skip_serializing_if = "Option::is_none")]
    pub src_mac_addr_end: Option<String>,
    /// String identifying a MAC address formatted in the hexadecimal notation according to clause 1.1 and clause 2.1 of RFC 7042.
    #[serde(rename = "destMacAddrEnd", skip_serializing_if = "Option::is_none")]
    pub dest_mac_addr_end: Option<String>,
}

impl EthFlowDescription {
    /// Identifies an Ethernet flow.
    pub fn new(eth_type: String) -> EthFlowDescription {
        EthFlowDescription {
            dest_mac_addr: None,
            eth_type,
            f_desc: None,
            f_dir: None,
            source_mac_addr: None,
            vlan_tags: None,
            src_mac_addr_end: None,
            dest_mac_addr_end: None,
        }
    }
}
