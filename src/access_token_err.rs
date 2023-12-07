use serde::{Deserialize, Serialize};

/// AccessTokenErr : Error returned in the access token response message

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessTokenErr {
    #[serde(rename = "error")]
    pub error: Error,
    #[serde(rename = "error_description", skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    #[serde(rename = "error_uri", skip_serializing_if = "Option::is_none")]
    pub error_uri: Option<String>,
}

impl AccessTokenErr {
    /// Error returned in the access token response message
    pub fn new(error: Error) -> AccessTokenErr {
        AccessTokenErr {
            error,
            error_description: None,
            error_uri: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Error {
    #[serde(rename = "invalid_request")]
    InvalidRequest,
    #[serde(rename = "invalid_client")]
    InvalidClient,
    #[serde(rename = "invalid_grant")]
    InvalidGrant,
    #[serde(rename = "unauthorized_client")]
    UnauthorizedClient,
    #[serde(rename = "unsupported_grant_type")]
    UnsupportedGrantType,
    #[serde(rename = "invalid_scope")]
    InvalidScope,
}

impl Default for Error {
    fn default() -> Error {
        Self::InvalidRequest
    }
}