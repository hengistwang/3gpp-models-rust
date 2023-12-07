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

/// PfdChangeNotification : Represents information related to a notification of PFD change.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PfdChangeNotification {
    /// String providing an application identifier.
    #[serde(rename = "applicationId")]
    pub application_id: String,
    #[serde(rename = "removalFlag", skip_serializing_if = "Option::is_none")]
    pub removal_flag: Option<bool>,
    #[serde(rename = "partialFlag", skip_serializing_if = "Option::is_none")]
    pub partial_flag: Option<bool>,
    #[serde(rename = "pfds", skip_serializing_if = "Option::is_none")]
    pub pfds: Option<Vec<crate::PfdContent>>,
}

impl PfdChangeNotification {
    /// Represents information related to a notification of PFD change.
    pub fn new(application_id: String) -> PfdChangeNotification {
        PfdChangeNotification {
            application_id,
            removal_flag: None,
            partial_flag: None,
            pfds: None,
        }
    }
}