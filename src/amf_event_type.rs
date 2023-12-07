use serde::{Deserialize, Serialize};

/// AmfEventType : Describes the supported event types of Namf_EventExposure Service

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmfEventType {}

impl AmfEventType {
    /// Describes the supported event types of Namf_EventExposure Service
    pub fn new() -> AmfEventType {
        AmfEventType {}
    }
}
