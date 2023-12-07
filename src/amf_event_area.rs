use serde::{Deserialize, Serialize};

/// AmfEventArea : Represents an area to be monitored by an AMF event

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmfEventArea {
    #[serde(rename = "presenceInfo", skip_serializing_if = "Option::is_none")]
    pub presence_info: Option<Box<crate::PresenceInfo>>,
    #[serde(rename = "ladnInfo", skip_serializing_if = "Option::is_none")]
    pub ladn_info: Option<Box<crate::LadnInfo>>,
    #[serde(rename = "sNssai", skip_serializing_if = "Option::is_none")]
    pub s_nssai: Option<Box<crate::Snssai>>,
    /// Contains the Identifier of the selected Network Slice instance
    #[serde(rename = "nsiId", skip_serializing_if = "Option::is_none")]
    pub nsi_id: Option<String>,
}

impl AmfEventArea {
    /// Represents an area to be monitored by an AMF event
    pub fn new() -> AmfEventArea {
        AmfEventArea {
            presence_info: None,
            ladn_info: None,
            s_nssai: None,
            nsi_id: None,
        }
    }
}
