use serde::{Deserialize, Serialize};

/// Model5GsUserStateInfo : Represents the 5GS User state of the UE for an access type

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Model5GsUserStateInfo {
    #[serde(rename = "5gsUserState")]
    pub param_5gs_user_state: Box<crate::Model5GsUserState>,
    #[serde(rename = "accessType")]
    pub access_type: crate::AccessType,
}

impl Model5GsUserStateInfo {
    /// Represents the 5GS User state of the UE for an access type
    pub fn new(
        param_5gs_user_state: crate::Model5GsUserState,
        access_type: crate::AccessType,
    ) -> Model5GsUserStateInfo {
        Model5GsUserStateInfo {
            param_5gs_user_state: Box::new(param_5gs_user_state),
            access_type,
        }
    }
}
