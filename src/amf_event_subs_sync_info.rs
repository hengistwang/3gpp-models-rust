use serde::{Deserialize, Serialize};

/// AmfEventSubsSyncInfo : AMF Event Subscriptions Information for synchronization

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmfEventSubsSyncInfo {
    #[serde(rename = "subscriptionList")]
    pub subscription_list: Vec<crate::AmfEventSubscriptionInfo>,
}

impl AmfEventSubsSyncInfo {
    /// AMF Event Subscriptions Information for synchronization
    pub fn new(subscription_list: Vec<crate::AmfEventSubscriptionInfo>) -> AmfEventSubsSyncInfo {
        AmfEventSubsSyncInfo { subscription_list }
    }
}
