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

/// ServiceName : Service names known to NRF

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceName {}

impl ServiceName {
    /// Service names known to NRF
    pub fn new() -> ServiceName {
        ServiceName {}
    }
}
