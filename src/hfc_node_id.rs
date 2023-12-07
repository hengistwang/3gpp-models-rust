use serde::{Deserialize, Serialize};

/// HfcNodeId : REpresents the HFC Node Identifer received over NGAP.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HfcNodeId {
    /// This IE represents the identifier of the HFC node Id as specified in CableLabs WR-TR-5WWC-ARCH. It is provisioned by the wireline operator as part of wireline operations and may contain up to six characters.
    #[serde(rename = "hfcNId")]
    pub hfc_nid: String,
}

impl HfcNodeId {
    /// REpresents the HFC Node Identifer received over NGAP.
    pub fn new(hfc_nid: String) -> HfcNodeId {
        HfcNodeId { hfc_nid }
    }
}
