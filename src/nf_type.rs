use serde::{Deserialize, Serialize};

/// NfType : NF types known to NRF

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NfType {}

impl NfType {
    /// NF types known to NRF
    pub fn new() -> NfType {
        NfType {}
    }
}
