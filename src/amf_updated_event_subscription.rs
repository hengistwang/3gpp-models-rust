use serde::{Deserialize, Serialize};

/// AmfUpdatedEventSubscription : Represents a successful update on an AMF Event Subscription

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmfUpdatedEventSubscription {
    #[serde(rename = "subscription")]
    pub subscription: Box<crate::AmfEventSubscription>,
    #[serde(rename = "reportList", skip_serializing_if = "Option::is_none")]
    pub report_list: Option<Vec<crate::AmfEventReport>>,
}

impl AmfUpdatedEventSubscription {
    /// Represents a successful update on an AMF Event Subscription
    pub fn new(subscription: crate::AmfEventSubscription) -> AmfUpdatedEventSubscription {
        AmfUpdatedEventSubscription {
            subscription: Box::new(subscription),
            report_list: None,
        }
    }
}
