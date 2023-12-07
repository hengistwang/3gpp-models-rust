use serde::{Deserialize, Serialize};

/// CmState : Describes the connection management state of a UE

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CmState {}

impl CmState {
    /// Describes the connection management state of a UE
    pub fn new() -> CmState {
        CmState {}
    }
}
