use serde::{Deserialize, Serialize};

/// AmfEvent : Describes an event to be subscribed

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmfEvent {
    #[serde(rename = "type")]
    pub r#type: Box<crate::AmfEventType>,
    #[serde(rename = "immediateFlag", skip_serializing_if = "Option::is_none")]
    pub immediate_flag: Option<bool>,
    #[serde(rename = "areaList", skip_serializing_if = "Option::is_none")]
    pub area_list: Option<Vec<crate::AmfEventArea>>,
    #[serde(rename = "locationFilterList", skip_serializing_if = "Option::is_none")]
    pub location_filter_list: Option<Vec<crate::LocationFilter>>,
    #[serde(rename = "refId", skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<i32>,
    #[serde(
        rename = "trafficDescriptorList",
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_descriptor_list: Option<Vec<crate::TrafficDescriptor>>,
    #[serde(rename = "reportUeReachable", skip_serializing_if = "Option::is_none")]
    pub report_ue_reachable: Option<bool>,
    #[serde(rename = "reachabilityFilter", skip_serializing_if = "Option::is_none")]
    pub reachability_filter: Option<Box<crate::ReachabilityFilter>>,
    #[serde(rename = "udmDetectInd", skip_serializing_if = "Option::is_none")]
    pub udm_detect_ind: Option<bool>,
    #[serde(rename = "maxReports", skip_serializing_if = "Option::is_none")]
    pub max_reports: Option<i32>,
    /// A map(list of key-value pairs) where praId serves as key.
    #[serde(rename = "presenceInfoList", skip_serializing_if = "Option::is_none")]
    pub presence_info_list: Option<::std::collections::HashMap<String, crate::PresenceInfo>>,
    /// indicating a time in seconds.
    #[serde(rename = "maxResponseTime", skip_serializing_if = "Option::is_none")]
    pub max_response_time: Option<i32>,
    #[serde(rename = "targetArea", skip_serializing_if = "Option::is_none")]
    pub target_area: Option<Box<crate::TargetArea>>,
    #[serde(rename = "snssaiFilter", skip_serializing_if = "Option::is_none")]
    pub snssai_filter: Option<Vec<crate::ExtSnssai>>,
    #[serde(rename = "ueInAreaFilter", skip_serializing_if = "Option::is_none")]
    pub ue_in_area_filter: Option<Box<crate::UeInAreaFilter>>,
    /// indicating a time in seconds.
    #[serde(rename = "minInterval", skip_serializing_if = "Option::is_none")]
    pub min_interval: Option<i32>,
    /// string with format 'date-time' as defined in OpenAPI.
    #[serde(rename = "nextReport", skip_serializing_if = "Option::is_none")]
    pub next_report: Option<String>,
    #[serde(rename = "idleStatusInd", skip_serializing_if = "Option::is_none")]
    pub idle_status_ind: Option<bool>,
    #[serde(rename = "dispersionArea", skip_serializing_if = "Option::is_none")]
    pub dispersion_area: Option<Box<crate::DispersionArea>>,
    /// string with format 'date-time' as defined in OpenAPI.
    #[serde(
        rename = "nextPeriodicReportTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_periodic_report_time: Option<String>,
}

impl AmfEvent {
    /// Describes an event to be subscribed
    pub fn new(r#type: crate::AmfEventType) -> AmfEvent {
        AmfEvent {
            r#type: Box::new(r#type),
            immediate_flag: None,
            area_list: None,
            location_filter_list: None,
            ref_id: None,
            traffic_descriptor_list: None,
            report_ue_reachable: None,
            reachability_filter: None,
            udm_detect_ind: None,
            max_reports: None,
            presence_info_list: None,
            max_response_time: None,
            target_area: None,
            snssai_filter: None,
            ue_in_area_filter: None,
            min_interval: None,
            next_report: None,
            idle_status_ind: None,
            dispersion_area: None,
            next_periodic_report_time: None,
        }
    }
}
