use serde::{Deserialize, Serialize};

/// AppliedSmccType : Possible values are: - DNN_CC: Indicates the DNN based congestion control. - SNSSAI_CC: Indicates the S-NSSAI based congestion control.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppliedSmccType {}

impl AppliedSmccType {
    /// Possible values are: - DNN_CC: Indicates the DNN based congestion control. - SNSSAI_CC: Indicates the S-NSSAI based congestion control.
    pub fn new() -> AppliedSmccType {
        AppliedSmccType {}
    }
}
