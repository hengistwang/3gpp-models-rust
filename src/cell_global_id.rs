use serde::{Deserialize, Serialize};

/// CellGlobalId : Contains a Cell Global Identification as defined in 3GPP TS 23.003, clause 4.3.1.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CellGlobalId {
    #[serde(rename = "plmnId")]
    pub plmn_id: Box<crate::PlmnId>,
    #[serde(rename = "lac")]
    pub lac: String,
    #[serde(rename = "cellId")]
    pub cell_id: String,
}

impl CellGlobalId {
    /// Contains a Cell Global Identification as defined in 3GPP TS 23.003, clause 4.3.1.
    pub fn new(plmn_id: crate::PlmnId, lac: String, cell_id: String) -> CellGlobalId {
        CellGlobalId {
            plmn_id: Box::new(plmn_id),
            lac,
            cell_id,
        }
    }
}
