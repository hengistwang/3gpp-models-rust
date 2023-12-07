use serde::{Deserialize, Serialize};

/// CmInfo : Represents the connection management state of a UE for an access type

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CmInfo {
    #[serde(rename = "cmState")]
    pub cm_state: Box<crate::CmState>,
    #[serde(rename = "accessType")]
    pub access_type: crate::AccessType,
}

impl CmInfo {
    /// Represents the connection management state of a UE for an access type
    pub fn new(cm_state: crate::CmState, access_type: crate::AccessType) -> CmInfo {
        CmInfo {
            cm_state: Box::new(cm_state),
            access_type,
        }
    }
}
