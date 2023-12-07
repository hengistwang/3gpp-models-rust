use serde::{Deserialize, Serialize};

/// MmTransactionSliceReportItem : UE MM Transaction Report Item per Slice

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MmTransactionSliceReportItem {
    #[serde(rename = "snssai", skip_serializing_if = "Option::is_none")]
    pub snssai: Option<Box<crate::Snssai>>,
    /// string with format 'date-time' as defined in OpenAPI.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "transactions")]
    pub transactions: i32,
}

impl MmTransactionSliceReportItem {
    /// UE MM Transaction Report Item per Slice
    pub fn new(timestamp: String, transactions: i32) -> MmTransactionSliceReportItem {
        MmTransactionSliceReportItem {
            snssai: None,
            timestamp,
            transactions,
        }
    }
}
