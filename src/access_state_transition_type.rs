/// AccessStateTransitionType : Access State Transition Type.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessStateTransitionType {}

impl AccessStateTransitionType {
    /// Access State Transition Type.
    pub fn new() -> AccessStateTransitionType {
        AccessStateTransitionType {}
    }
}
