use serde::{Deserialize, Serialize};
/*
 * Nnef_PFDmanagement Service API
 *
 * Packet Flow Description Management Service.   © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC).   All rights reserved.
 *
 * The version of the OpenAPI document: 1.2.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// PfdChangeReport : Represents an error report on PFD change.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PfdChangeReport {
    #[serde(rename = "pfdError")]
    pub pfd_error: Box<crate::ProblemDetails>,
    #[serde(rename = "applicationId")]
    pub application_id: Vec<String>,
}

impl PfdChangeReport {
    /// Represents an error report on PFD change.
    pub fn new(pfd_error: crate::ProblemDetails, application_id: Vec<String>) -> PfdChangeReport {
        PfdChangeReport {
            pfd_error: Box::new(pfd_error),
            application_id,
        }
    }
}
