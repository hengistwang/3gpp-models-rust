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

/// TrafficDescriptor : Represents the Traffic Descriptor

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrafficDescriptor {
    /// String representing a Data Network as defined in clause 9A of 3GPP TS 23.003;  it shall contain either a DNN Network Identifier, or a full DNN with both the Network  Identifier and Operator Identifier, as specified in 3GPP TS 23.003 clause 9.1.1 and 9.1.2. It shall be coded as string in which the labels are separated by dots  (e.g. \"Label1.Label2.Label3\").
    #[serde(rename = "dnn", skip_serializing_if = "Option::is_none")]
    pub dnn: Option<String>,
    #[serde(rename = "sNssai", skip_serializing_if = "Option::is_none")]
    pub s_nssai: Option<Box<crate::Snssai>>,
    #[serde(
        rename = "dddTrafficDescriptorList",
        skip_serializing_if = "Option::is_none"
    )]
    pub ddd_traffic_descriptor_list: Option<Vec<crate::DddTrafficDescriptor>>,
}

impl TrafficDescriptor {
    /// Represents the Traffic Descriptor
    pub fn new() -> TrafficDescriptor {
        TrafficDescriptor {
            dnn: None,
            s_nssai: None,
            ddd_traffic_descriptor_list: None,
        }
    }
}
