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

/// PfdDataForApp : Represents the PFDs for an application identifier.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PfdDataForApp {
    /// String providing an application identifier.
    #[serde(rename = "applicationId")]
    pub application_id: String,
    #[serde(rename = "pfds", skip_serializing_if = "Option::is_none")]
    pub pfds: Option<Vec<crate::PfdContent>>,
    /// string with format 'date-time' as defined in OpenAPI.
    #[serde(rename = "cachingTime", skip_serializing_if = "Option::is_none")]
    pub caching_time: Option<String>,
    /// indicating a time in seconds.
    #[serde(rename = "cachingTimer", skip_serializing_if = "Option::is_none")]
    pub caching_timer: Option<i32>,
    /// string with format 'date-time' as defined in OpenAPI.
    #[serde(rename = "pfdTimestamp", skip_serializing_if = "Option::is_none")]
    pub pfd_timestamp: Option<String>,
    #[serde(rename = "partialFlag", skip_serializing_if = "Option::is_none")]
    pub partial_flag: Option<bool>,
    /// A string used to indicate the features supported by an API that is used as defined in clause  6.6 in 3GPP TS 29.500. The string shall contain a bitmask indicating supported features in  hexadecimal representation Each character in the string shall take a value of \"0\" to \"9\",  \"a\" to \"f\" or \"A\" to \"F\" and shall represent the support of 4 features as described in  table 5.2.2-3. The most significant character representing the highest-numbered features shall  appear first in the string, and the character representing features 1 to 4 shall appear last  in the string. The list of features and their numbering (starting with 1) are defined  separately for each API. If the string contains a lower number of characters than there are  defined features for an API, all features that would be represented by characters that are not  present in the string are not supported.
    #[serde(rename = "supportedFeatures", skip_serializing_if = "Option::is_none")]
    pub supported_features: Option<String>,
}

impl PfdDataForApp {
    /// Represents the PFDs for an application identifier.
    pub fn new(application_id: String) -> PfdDataForApp {
        PfdDataForApp {
            application_id,
            pfds: None,
            caching_time: None,
            caching_timer: None,
            pfd_timestamp: None,
            partial_flag: None,
            supported_features: None,
        }
    }
}
