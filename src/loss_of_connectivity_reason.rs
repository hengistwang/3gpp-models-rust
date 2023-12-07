use serde::{Deserialize, Serialize};

/// LossOfConnectivityReason : Describes the reason for loss of connectivity

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LossOfConnectivityReason {}

impl LossOfConnectivityReason {
    /// Describes the reason for loss of connectivity
    pub fn new() -> LossOfConnectivityReason {
        LossOfConnectivityReason {}
    }
}
