use serde::{Deserialize, Serialize};

/// LocationAreaId : Contains a Location area identification as defined in 3GPP TS 23.003, clause 4.1.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocationAreaId {
    #[serde(rename = "plmnId")]
    pub plmn_id: Box<crate::PlmnId>,
    /// Location Area Code.
    #[serde(rename = "lac")]
    pub lac: String,
}

impl LocationAreaId {
    /// Contains a Location area identification as defined in 3GPP TS 23.003, clause 4.1.
    pub fn new(plmn_id: crate::PlmnId, lac: String) -> LocationAreaId {
        LocationAreaId {
            plmn_id: Box::new(plmn_id),
            lac,
        }
    }
}
