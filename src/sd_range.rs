use serde::{Deserialize, Serialize};
/*
 * Namf_EventExposure
 *
 * AMF Event Exposure Service.   © 2023, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved.
 *
 * The version of the OpenAPI document: 1.2.3
 *
 * Generated by: https://openapi-generator.tech
 */

/// SdRange : A range of SDs (Slice Differentiators)

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SdRange {
    /// First value identifying the start of an SD range. This string shall be formatted as specified for the sd attribute of the Snssai data type in clause 5.4.4.2.
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// Last value identifying the end of an SD range. This string shall be formatted as specified for the sd attribute of the Snssai data type in clause 5.4.4.2.
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
}

impl SdRange {
    /// A range of SDs (Slice Differentiators)
    pub fn new() -> SdRange {
        SdRange {
            start: None,
            end: None,
        }
    }
}
