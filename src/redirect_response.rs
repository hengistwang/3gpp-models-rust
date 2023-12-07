use serde::{Deserialize, Serialize};

/// RedirectResponse : The response shall include a Location header field containing a different URI  (pointing to a different URI of an other service instance), or the same URI if a request  is redirected to the same target resource via a different SCP.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedirectResponse {
    #[serde(rename = "cause", skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// String providing an URI formatted according to RFC 3986.
    #[serde(rename = "targetScp", skip_serializing_if = "Option::is_none")]
    pub target_scp: Option<String>,
    /// String providing an URI formatted according to RFC 3986.
    #[serde(rename = "targetSepp", skip_serializing_if = "Option::is_none")]
    pub target_sepp: Option<String>,
}

impl RedirectResponse {
    /// The response shall include a Location header field containing a different URI  (pointing to a different URI of an other service instance), or the same URI if a request  is redirected to the same target resource via a different SCP.
    pub fn new() -> RedirectResponse {
        RedirectResponse {
            cause: None,
            target_scp: None,
            target_sepp: None,
        }
    }
}
