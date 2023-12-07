use serde::{Deserialize, Serialize};

/// AmfEventTrigger : Describes how AMF should generate the report for the event

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmfEventTrigger {}

impl AmfEventTrigger {
    /// Describes how AMF should generate the report for the event
    pub fn new() -> AmfEventTrigger {
        AmfEventTrigger {}
    }
}
