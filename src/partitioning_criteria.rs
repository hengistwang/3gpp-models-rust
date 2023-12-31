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

/// PartitioningCriteria : Possible values are: - \"TAC\": Type Allocation Code - \"SUBPLMN\": Subscriber PLMN ID - \"GEOAREA\": Geographical area, i.e. list(s) of TAI(s) - \"SNSSAI\": S-NSSAI - \"DNN\": DNN

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartitioningCriteria {}

impl PartitioningCriteria {
    /// Possible values are: - \"TAC\": Type Allocation Code - \"SUBPLMN\": Subscriber PLMN ID - \"GEOAREA\": Geographical area, i.e. list(s) of TAI(s) - \"SNSSAI\": S-NSSAI - \"DNN\": DNN
    pub fn new() -> PartitioningCriteria {
        PartitioningCriteria {}
    }
}
