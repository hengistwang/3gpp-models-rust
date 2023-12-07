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

/// Tai : Contains the tracking area identity as described in 3GPP 23.003

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tai {
    #[serde(rename = "plmnId")]
    pub plmn_id: Box<crate::PlmnId>,
    /// 2 or 3-octet string identifying a tracking area code as specified in clause 9.3.3.10  of 3GPP TS 38.413, in hexadecimal representation. Each character in the string shall  take a value of \"0\" to \"9\", \"a\" to \"f\" or \"A\" to \"F\" and shall represent 4 bits. The most significant character representing the 4 most significant bits of the TAC shall  appear first in the string, and the character representing the 4 least significant bit  of the TAC shall appear last in the string.  
    #[serde(rename = "tac")]
    pub tac: String,
    /// This represents the Network Identifier, which together with a PLMN ID is used to identify an SNPN (see 3GPP TS 23.003 and 3GPP TS 23.501 clause 5.30.2.1).  
    #[serde(rename = "nid", skip_serializing_if = "Option::is_none")]
    pub nid: Option<String>,
}

impl Tai {
    /// Contains the tracking area identity as described in 3GPP 23.003
    pub fn new(plmn_id: crate::PlmnId, tac: String) -> Tai {
        Tai {
            plmn_id: Box::new(plmn_id),
            tac,
            nid: None,
        }
    }
}
