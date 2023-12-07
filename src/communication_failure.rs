use serde::{Deserialize, Serialize};

/// CommunicationFailure : Describes a communication failure detected by AMF

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommunicationFailure {
    #[serde(rename = "nasReleaseCode", skip_serializing_if = "Option::is_none")]
    pub nas_release_code: Option<String>,
    #[serde(rename = "ranReleaseCode", skip_serializing_if = "Option::is_none")]
    pub ran_release_code: Option<Box<crate::NgApCause>>,
}

impl CommunicationFailure {
    /// Describes a communication failure detected by AMF
    pub fn new() -> CommunicationFailure {
        CommunicationFailure {
            nas_release_code: None,
            ran_release_code: None,
        }
    }
}
