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

/// UeAccessBehaviorReportItem : Report Item for UE Access Behavior Trends event.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UeAccessBehaviorReportItem {
    #[serde(rename = "stateTransitionType")]
    pub state_transition_type: Box<crate::AccessStateTransitionType>,
    /// indicating a time in seconds.
    #[serde(rename = "spacing")]
    pub spacing: i32,
    /// indicating a time in seconds.
    #[serde(rename = "duration")]
    pub duration: i32,
}

impl UeAccessBehaviorReportItem {
    /// Report Item for UE Access Behavior Trends event.
    pub fn new(
        state_transition_type: crate::AccessStateTransitionType,
        spacing: i32,
        duration: i32,
    ) -> UeAccessBehaviorReportItem {
        UeAccessBehaviorReportItem {
            state_transition_type: Box::new(state_transition_type),
            spacing,
            duration,
        }
    }
}