use serde::{Deserialize, Serialize};

/// NgApCause : Represents the NGAP cause.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NgApCause {
    /// Unsigned Integer, i.e. only value 0 and integers above 0 are permissible.
    #[serde(rename = "group")]
    pub group: i32,
    /// Unsigned Integer, i.e. only value 0 and integers above 0 are permissible.
    #[serde(rename = "value")]
    pub value: i32,
}

impl NgApCause {
    /// Represents the NGAP cause.
    pub fn new(group: i32, value: i32) -> NgApCause {
        NgApCause { group, value }
    }
}
