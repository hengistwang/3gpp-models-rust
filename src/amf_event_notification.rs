use serde::{Deserialize, Serialize};

/// AmfEventNotification : Data within a AMF Event Notification request

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmfEventNotification {
    #[serde(
        rename = "notifyCorrelationId",
        skip_serializing_if = "Option::is_none"
    )]
    pub notify_correlation_id: Option<String>,
    #[serde(
        rename = "subsChangeNotifyCorrelationId",
        skip_serializing_if = "Option::is_none"
    )]
    pub subs_change_notify_correlation_id: Option<String>,
    #[serde(rename = "reportList", skip_serializing_if = "Option::is_none")]
    pub report_list: Option<Vec<crate::AmfEventReport>>,
    #[serde(rename = "eventSubsSyncInfo", skip_serializing_if = "Option::is_none")]
    pub event_subs_sync_info: Option<Box<crate::AmfEventSubsSyncInfo>>,
}

impl AmfEventNotification {
    /// Data within a AMF Event Notification request
    pub fn new() -> AmfEventNotification {
        AmfEventNotification {
            notify_correlation_id: None,
            subs_change_notify_correlation_id: None,
            report_list: None,
            event_subs_sync_info: None,
        }
    }
}
