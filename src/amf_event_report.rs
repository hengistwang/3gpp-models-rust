use serde::{Deserialize, Serialize};

/// AmfEventReport : Represents a report triggered by a subscribed event type

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmfEventReport {
    #[serde(rename = "type")]
    pub r#type: Box<crate::AmfEventType>,
    #[serde(rename = "state")]
    pub state: Box<crate::AmfEventState>,
    /// string with format 'date-time' as defined in OpenAPI.
    #[serde(rename = "timeStamp")]
    pub time_stamp: String,
    /// String providing an URI formatted according to RFC 3986.
    #[serde(rename = "subscriptionId", skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "anyUe", skip_serializing_if = "Option::is_none")]
    pub any_ue: Option<bool>,
    /// String identifying a Supi that shall contain either an IMSI, a network specific identifier, a Global Cable Identifier (GCI) or a Global Line Identifier (GLI) as specified in clause  2.2A of 3GPP TS 23.003. It shall be formatted as follows  - for an IMSI \"imsi-<imsi>\", where <imsi> shall be formatted according to clause 2.2    of 3GPP TS 23.003 that describes an IMSI.  - for a network specific identifier \"nai-<nai>, where <nai> shall be formatted    according to clause 28.7.2 of 3GPP TS 23.003 that describes an NAI.  - for a GCI \"gci-<gci>\", where <gci> shall be formatted according to clause 28.15.2    of 3GPP TS 23.003.  - for a GLI \"gli-<gli>\", where <gli> shall be formatted according to clause 28.16.2 of    3GPP TS 23.003.To enable that the value is used as part of an URI, the string shall    only contain characters allowed according to the \"lower-with-hyphen\" naming convention    defined in 3GPP TS 29.501.
    #[serde(rename = "supi", skip_serializing_if = "Option::is_none")]
    pub supi: Option<String>,
    #[serde(rename = "areaList", skip_serializing_if = "Option::is_none")]
    pub area_list: Option<Vec<crate::AmfEventArea>>,
    #[serde(rename = "refId", skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<i32>,
    /// String identifying a Gpsi shall contain either an External Id or an MSISDN.  It shall be formatted as follows -External Identifier= \"extid-'extid', where 'extid'  shall be formatted according to clause 19.7.2 of 3GPP TS 23.003 that describes an  External Identifier.  
    #[serde(rename = "gpsi", skip_serializing_if = "Option::is_none")]
    pub gpsi: Option<String>,
    /// String representing a Permanent Equipment Identifier that may contain - an IMEI or IMEISV, as  specified in clause 6.2 of 3GPP TS 23.003; a MAC address for a 5G-RG or FN-RG via  wireline  access, with an indication that this address cannot be trusted for regulatory purpose if this  address cannot be used as an Equipment Identifier of the FN-RG, as specified in clause 4.7.7  of 3GPP TS23.316. Examples are imei-012345678901234 or imeisv-0123456789012345.  
    #[serde(rename = "pei", skip_serializing_if = "Option::is_none")]
    pub pei: Option<String>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<crate::UserLocation>>,
    #[serde(rename = "additionalLocation", skip_serializing_if = "Option::is_none")]
    pub additional_location: Option<Box<crate::UserLocation>>,
    /// String with format \"time-numoffset\" optionally appended by \"daylightSavingTime\", where  - \"time-numoffset\" shall represent the time zone adjusted for daylight saving time and be    encoded as time-numoffset as defined in clause 5.6 of IETF RFC 3339;  - \"daylightSavingTime\" shall represent the adjustment that has been made and shall be    encoded as \"+1\" or \"+2\" for a +1 or +2 hours adjustment.   The example is for 8 hours behind UTC, +1 hour adjustment for Daylight Saving Time.
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "accessTypeList", skip_serializing_if = "Option::is_none")]
    pub access_type_list: Option<Vec<crate::AccessType>>,
    #[serde(rename = "rmInfoList", skip_serializing_if = "Option::is_none")]
    pub rm_info_list: Option<Vec<crate::RmInfo>>,
    #[serde(rename = "cmInfoList", skip_serializing_if = "Option::is_none")]
    pub cm_info_list: Option<Vec<crate::CmInfo>>,
    #[serde(rename = "reachability", skip_serializing_if = "Option::is_none")]
    pub reachability: Option<Box<crate::UeReachability>>,
    #[serde(rename = "commFailure", skip_serializing_if = "Option::is_none")]
    pub comm_failure: Option<Box<crate::CommunicationFailure>>,
    #[serde(
        rename = "lossOfConnectReason",
        skip_serializing_if = "Option::is_none"
    )]
    pub loss_of_connect_reason: Option<Box<crate::LossOfConnectivityReason>>,
    #[serde(rename = "numberOfUes", skip_serializing_if = "Option::is_none")]
    pub number_of_ues: Option<i32>,
    #[serde(rename = "5gsUserStateList", skip_serializing_if = "Option::is_none")]
    pub param_5gs_user_state_list: Option<Vec<crate::Model5GsUserStateInfo>>,
    #[serde(rename = "typeCode", skip_serializing_if = "Option::is_none")]
    pub type_code: Option<String>,
    #[serde(rename = "registrationNumber", skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<i32>,
    /// string with format 'date-time' as defined in OpenAPI.
    #[serde(
        rename = "maxAvailabilityTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_availability_time: Option<String>,
    #[serde(rename = "ueIdExt", skip_serializing_if = "Option::is_none")]
    pub ue_id_ext: Option<Vec<crate::UeidExt>>,
    #[serde(rename = "snssaiTaiList", skip_serializing_if = "Option::is_none")]
    pub snssai_tai_list: Option<Vec<crate::SnssaiTaiMapping>>,
    #[serde(
        rename = "idleStatusIndication",
        skip_serializing_if = "Option::is_none"
    )]
    pub idle_status_indication: Option<Box<crate::IdleStatusIndication>>,
    #[serde(
        rename = "ueAccessBehaviorTrends",
        skip_serializing_if = "Option::is_none"
    )]
    pub ue_access_behavior_trends: Option<Vec<crate::UeAccessBehaviorReportItem>>,
    #[serde(rename = "ueLocationTrends", skip_serializing_if = "Option::is_none")]
    pub ue_location_trends: Option<Vec<crate::UeLocationTrendsReportItem>>,
    #[serde(
        rename = "mmTransLocationReportList",
        skip_serializing_if = "Option::is_none"
    )]
    pub mm_trans_location_report_list: Option<Vec<crate::MmTransactionLocationReportItem>>,
    #[serde(
        rename = "mmTransSliceReportList",
        skip_serializing_if = "Option::is_none"
    )]
    pub mm_trans_slice_report_list: Option<Vec<crate::MmTransactionSliceReportItem>>,
}

impl AmfEventReport {
    /// Represents a report triggered by a subscribed event type
    pub fn new(
        r#type: crate::AmfEventType,
        state: crate::AmfEventState,
        time_stamp: String,
    ) -> AmfEventReport {
        AmfEventReport {
            r#type: Box::new(r#type),
            state: Box::new(state),
            time_stamp,
            subscription_id: None,
            any_ue: None,
            supi: None,
            area_list: None,
            ref_id: None,
            gpsi: None,
            pei: None,
            location: None,
            additional_location: None,
            timezone: None,
            access_type_list: None,
            rm_info_list: None,
            cm_info_list: None,
            reachability: None,
            comm_failure: None,
            loss_of_connect_reason: None,
            number_of_ues: None,
            param_5gs_user_state_list: None,
            type_code: None,
            registration_number: None,
            max_availability_time: None,
            ue_id_ext: None,
            snssai_tai_list: None,
            idle_status_indication: None,
            ue_access_behavior_trends: None,
            ue_location_trends: None,
            mm_trans_location_report_list: None,
            mm_trans_slice_report_list: None,
        }
    }
}
