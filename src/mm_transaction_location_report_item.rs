use serde::{Deserialize, Serialize};

/// MmTransactionLocationReportItem : UE MM Transaction Report Item per Location

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MmTransactionLocationReportItem {
    #[serde(rename = "tai", skip_serializing_if = "Option::is_none")]
    pub tai: Option<Box<crate::Tai>>,
    #[serde(rename = "ncgi", skip_serializing_if = "Option::is_none")]
    pub ncgi: Option<Box<crate::Ncgi>>,
    #[serde(rename = "ecgi", skip_serializing_if = "Option::is_none")]
    pub ecgi: Option<Box<crate::Ecgi>>,
    #[serde(rename = "n3gaLocation", skip_serializing_if = "Option::is_none")]
    pub n3ga_location: Option<Box<crate::N3gaLocation>>,
    /// string with format 'date-time' as defined in OpenAPI.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "transactions")]
    pub transactions: i32,
}

impl MmTransactionLocationReportItem {
    /// UE MM Transaction Report Item per Location
    pub fn new(timestamp: String, transactions: i32) -> MmTransactionLocationReportItem {
        MmTransactionLocationReportItem {
            tai: None,
            ncgi: None,
            ecgi: None,
            n3ga_location: None,
            timestamp,
            transactions,
        }
    }
}
