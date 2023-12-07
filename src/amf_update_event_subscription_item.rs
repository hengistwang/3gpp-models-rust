use serde::{Deserialize, Serialize};

/// AmfUpdateEventSubscriptionItem : Document describing the modification(s) to an AMF Event Subscription

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmfUpdateEventSubscriptionItem {
    #[serde(rename = "op")]
    pub op: Op,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<crate::AmfEvent>>,
    #[serde(rename = "presenceInfo", skip_serializing_if = "Option::is_none")]
    pub presence_info: Option<Box<crate::PresenceInfo>>,
    #[serde(rename = "excludeSupiList", skip_serializing_if = "Option::is_none")]
    pub exclude_supi_list: Option<Vec<String>>,
    #[serde(rename = "excludeGpsiList", skip_serializing_if = "Option::is_none")]
    pub exclude_gpsi_list: Option<Vec<String>>,
    #[serde(rename = "includeSupiList", skip_serializing_if = "Option::is_none")]
    pub include_supi_list: Option<Vec<String>>,
    #[serde(rename = "includeGpsiList", skip_serializing_if = "Option::is_none")]
    pub include_gpsi_list: Option<Vec<String>>,
}

impl AmfUpdateEventSubscriptionItem {
    /// Document describing the modification(s) to an AMF Event Subscription
    pub fn new(op: Op, path: String) -> AmfUpdateEventSubscriptionItem {
        AmfUpdateEventSubscriptionItem {
            op,
            path,
            value: None,
            presence_info: None,
            exclude_supi_list: None,
            exclude_gpsi_list: None,
            include_supi_list: None,
            include_gpsi_list: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Op {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "replace")]
    Replace,
}

impl Default for Op {
    fn default() -> Op {
        Self::Add
    }
}
