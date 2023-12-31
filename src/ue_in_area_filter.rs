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

/// UeInAreaFilter : Additional filters for UE in Area Report event

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UeInAreaFilter {
    #[serde(rename = "ueType", skip_serializing_if = "Option::is_none")]
    pub ue_type: Option<Box<crate::UeType>>,
    #[serde(rename = "aerialSrvDnnInd", skip_serializing_if = "Option::is_none")]
    pub aerial_srv_dnn_ind: Option<bool>,
}

impl UeInAreaFilter {
    /// Additional filters for UE in Area Report event
    pub fn new() -> UeInAreaFilter {
        UeInAreaFilter {
            ue_type: None,
            aerial_srv_dnn_ind: None,
        }
    }
}
