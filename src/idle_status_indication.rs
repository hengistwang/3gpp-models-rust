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

/// IdleStatusIndication : Represents the idle status indication.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdleStatusIndication {
    /// string with format 'date-time' as defined in OpenAPI.
    #[serde(rename = "timeStamp", skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    /// indicating a time in seconds.
    #[serde(rename = "activeTime", skip_serializing_if = "Option::is_none")]
    pub active_time: Option<i32>,
    /// indicating a time in seconds.
    #[serde(rename = "subsRegTimer", skip_serializing_if = "Option::is_none")]
    pub subs_reg_timer: Option<i32>,
    #[serde(rename = "edrxCycleLength", skip_serializing_if = "Option::is_none")]
    pub edrx_cycle_length: Option<i32>,
    #[serde(
        rename = "suggestedNumOfDlPackets",
        skip_serializing_if = "Option::is_none"
    )]
    pub suggested_num_of_dl_packets: Option<i32>,
}

impl IdleStatusIndication {
    /// Represents the idle status indication.
    pub fn new() -> IdleStatusIndication {
        IdleStatusIndication {
            time_stamp: None,
            active_time: None,
            subs_reg_timer: None,
            edrx_cycle_length: None,
            suggested_num_of_dl_packets: None,
        }
    }
}
