use serde::{Deserialize, Serialize};

/// Model5GsUserState : Describes the 5GS User State of a UE

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Model5GsUserState {}

impl Model5GsUserState {
    /// Describes the 5GS User State of a UE
    pub fn new() -> Model5GsUserState {
        Model5GsUserState {}
    }
}
