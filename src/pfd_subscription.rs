use serde::{Deserialize, Serialize};

/// PfdSubscription : Represents a PFD subscription.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PfdSubscription {
    #[serde(rename = "applicationIds", skip_serializing_if = "Option::is_none")]
    pub application_ids: Option<Vec<String>>,
    /// String providing an URI formatted according to RFC 3986.
    #[serde(rename = "notifyUri")]
    pub notify_uri: String,
    /// A string used to indicate the features supported by an API that is used as defined in clause  6.6 in 3GPP TS 29.500. The string shall contain a bitmask indicating supported features in  hexadecimal representation Each character in the string shall take a value of \"0\" to \"9\",  \"a\" to \"f\" or \"A\" to \"F\" and shall represent the support of 4 features as described in  table 5.2.2-3. The most significant character representing the highest-numbered features shall  appear first in the string, and the character representing features 1 to 4 shall appear last  in the string. The list of features and their numbering (starting with 1) are defined  separately for each API. If the string contains a lower number of characters than there are  defined features for an API, all features that would be represented by characters that are not  present in the string are not supported.
    #[serde(rename = "supportedFeatures")]
    pub supported_features: String,
}

impl PfdSubscription {
    /// Represents a PFD subscription.
    pub fn new(notify_uri: String, supported_features: String) -> PfdSubscription {
        PfdSubscription {
            application_ids: None,
            notify_uri,
            supported_features,
        }
    }
}
