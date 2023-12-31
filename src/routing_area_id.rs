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

/// RoutingAreaId : Contains a Routing Area Identification as defined in 3GPP TS 23.003, clause 4.2.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoutingAreaId {
    #[serde(rename = "plmnId")]
    pub plmn_id: Box<crate::PlmnId>,
    /// Location Area Code
    #[serde(rename = "lac")]
    pub lac: String,
    /// Routing Area Code
    #[serde(rename = "rac")]
    pub rac: String,
}

impl RoutingAreaId {
    /// Contains a Routing Area Identification as defined in 3GPP TS 23.003, clause 4.2.
    pub fn new(plmn_id: crate::PlmnId, lac: String, rac: String) -> RoutingAreaId {
        RoutingAreaId {
            plmn_id: Box::new(plmn_id),
            lac,
            rac,
        }
    }
}
