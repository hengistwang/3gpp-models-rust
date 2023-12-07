use serde::{Deserialize, Serialize};

/// ApplicationForPfdRequest : Contains the application identifier(s) for the PFD(s) request.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationForPfdRequest {
    /// String providing an application identifier.
    #[serde(rename = "applicationId")]
    pub application_id: String,
    /// string with format 'date-time' as defined in OpenAPI.
    #[serde(rename = "pfdTimestamp", skip_serializing_if = "Option::is_none")]
    pub pfd_timestamp: Option<String>,
}

impl ApplicationForPfdRequest {
    /// Contains the application identifier(s) for the PFD(s) request.
    pub fn new(application_id: String) -> ApplicationForPfdRequest {
        ApplicationForPfdRequest {
            application_id,
            pfd_timestamp: None,
        }
    }
}
