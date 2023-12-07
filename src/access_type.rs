use serde::{Deserialize, Serialize};

/// AccessType : Indicates whether the access is  via 3GPP or via non-3GPP.

/// Indicates whether the access is  via 3GPP or via non-3GPP.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccessType {
    #[serde(rename = "3GPP_ACCESS")]
    Variant3GppAccess,
    #[serde(rename = "NON_3GPP_ACCESS")]
    Non3GppAccess,
}

impl ToString for AccessType {
    fn to_string(&self) -> String {
        match self {
            Self::Variant3GppAccess => String::from("3GPP_ACCESS"),
            Self::Non3GppAccess => String::from("NON_3GPP_ACCESS"),
        }
    }
}

impl Default for AccessType {
    fn default() -> AccessType {
        Self::Variant3GppAccess
    }
}
