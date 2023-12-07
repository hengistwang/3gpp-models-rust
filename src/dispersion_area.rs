use serde::{Deserialize, Serialize};

/// DispersionArea : Dispersion Area

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DispersionArea {
    #[serde(rename = "taiList", skip_serializing_if = "Option::is_none")]
    pub tai_list: Option<Vec<crate::Tai>>,
    #[serde(rename = "ncgiList", skip_serializing_if = "Option::is_none")]
    pub ncgi_list: Option<Vec<crate::Ncgi>>,
    #[serde(rename = "ecgiList", skip_serializing_if = "Option::is_none")]
    pub ecgi_list: Option<Vec<crate::Ecgi>>,
    #[serde(rename = "n3gaInd", skip_serializing_if = "Option::is_none")]
    pub n3ga_ind: Option<bool>,
}

impl DispersionArea {
    /// Dispersion Area
    pub fn new() -> DispersionArea {
        DispersionArea {
            tai_list: None,
            ncgi_list: None,
            ecgi_list: None,
            n3ga_ind: None,
        }
    }
}
