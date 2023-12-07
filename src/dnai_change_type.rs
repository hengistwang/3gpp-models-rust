use serde::{Deserialize, Serialize};
/*
 * Nsmf_EventExposure
 *
 * Session Management Event Exposure Service.   © 2023, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved.
 *
 * The version of the OpenAPI document: 1.2.2
 *
 * Generated by: https://openapi-generator.tech
 */

/// DnaiChangeType : Possible values are: - EARLY: Early notification of UP path reconfiguration. - EARLY_LATE: Early and late notification of UP path reconfiguration. This value shall   only be present in the subscription to the DNAI change event. - LATE: Late notification of UP path reconfiguration.  

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnaiChangeType {}

impl DnaiChangeType {
    /// Possible values are: - EARLY: Early notification of UP path reconfiguration. - EARLY_LATE: Early and late notification of UP path reconfiguration. This value shall   only be present in the subscription to the DNAI change event. - LATE: Late notification of UP path reconfiguration.  
    pub fn new() -> DnaiChangeType {
        DnaiChangeType {}
    }
}