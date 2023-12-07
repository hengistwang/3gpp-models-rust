use serde::{Deserialize, Serialize};

/// AmfEventState : Represents the state of a subscribed event

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmfEventState {
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "remainReports", skip_serializing_if = "Option::is_none")]
    pub remain_reports: Option<i32>,
    /// indicating a time in seconds.
    #[serde(rename = "remainDuration", skip_serializing_if = "Option::is_none")]
    pub remain_duration: Option<i32>,
}

impl AmfEventState {
    /// Represents the state of a subscribed event
    pub fn new(active: bool) -> AmfEventState {
        AmfEventState {
            active,
            remain_reports: None,
            remain_duration: None,
        }
    }
}
