use serde::{Deserialize, Serialize};

/// AmfUpdateEventOptionItem : Document describing the modifications to AMF event subscription options

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmfUpdateEventOptionItem {
    #[serde(rename = "op")]
    pub op: Op,
    #[serde(rename = "path")]
    pub path: String,
    /// string with format 'date-time' as defined in OpenAPI.
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "notifFlag", skip_serializing_if = "Option::is_none")]
    pub notif_flag: Option<Box<crate::NotificationFlag>>,
}

impl AmfUpdateEventOptionItem {
    /// Document describing the modifications to AMF event subscription options
    pub fn new(op: Op, path: String, value: String) -> AmfUpdateEventOptionItem {
        AmfUpdateEventOptionItem {
            op,
            path,
            value,
            notif_flag: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Op {
    #[serde(rename = "replace")]
    Replace,
}

impl Default for Op {
    fn default() -> Op {
        Self::Replace
    }
}
