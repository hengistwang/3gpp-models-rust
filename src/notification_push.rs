use serde::{Deserialize, Serialize};

/// NotificationPush : Represents the information to be used by the NF service consumer to retrieve the PFDs and/or remove the PFDs of the applicable application identifier(s).

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationPush {
    #[serde(rename = "appIds")]
    pub app_ids: Vec<String>,
    /// indicating a time in seconds.
    #[serde(rename = "allowedDelay", skip_serializing_if = "Option::is_none")]
    pub allowed_delay: Option<i32>,
    #[serde(rename = "pfdOp", skip_serializing_if = "Option::is_none")]
    pub pfd_op: Option<Box<crate::PfdOperation>>,
}

impl NotificationPush {
    /// Represents the information to be used by the NF service consumer to retrieve the PFDs and/or remove the PFDs of the applicable application identifier(s).
    pub fn new(app_ids: Vec<String>) -> NotificationPush {
        NotificationPush {
            app_ids,
            allowed_delay: None,
            pfd_op: None,
        }
    }
}
