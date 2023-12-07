use serde::{Deserialize, Serialize};

/// NsmfEventExposure : Represents an Individual SMF Notification Subscription resource. The serviveName property corresponds to the serviceName in the main body of the specification.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NsmfEventExposure {
    /// String identifying a Supi that shall contain either an IMSI, a network specific identifier, a Global Cable Identifier (GCI) or a Global Line Identifier (GLI) as specified in clause  2.2A of 3GPP TS 23.003. It shall be formatted as follows  - for an IMSI \"imsi-<imsi>\", where <imsi> shall be formatted according to clause 2.2    of 3GPP TS 23.003 that describes an IMSI.  - for a network specific identifier \"nai-<nai>, where <nai> shall be formatted    according to clause 28.7.2 of 3GPP TS 23.003 that describes an NAI.  - for a GCI \"gci-<gci>\", where <gci> shall be formatted according to clause 28.15.2    of 3GPP TS 23.003.  - for a GLI \"gli-<gli>\", where <gli> shall be formatted according to clause 28.16.2 of    3GPP TS 23.003.To enable that the value is used as part of an URI, the string shall    only contain characters allowed according to the \"lower-with-hyphen\" naming convention    defined in 3GPP TS 29.501.
    #[serde(rename = "supi", skip_serializing_if = "Option::is_none")]
    pub supi: Option<String>,
    /// String identifying a Gpsi shall contain either an External Id or an MSISDN.  It shall be formatted as follows -External Identifier= \"extid-'extid', where 'extid'  shall be formatted according to clause 19.7.2 of 3GPP TS 23.003 that describes an  External Identifier.  
    #[serde(rename = "gpsi", skip_serializing_if = "Option::is_none")]
    pub gpsi: Option<String>,
    /// Any UE indication. This IE shall be present if the event subscription is applicable to  any UE. Default value \"false\" is used, if not present.
    #[serde(rename = "anyUeInd", skip_serializing_if = "Option::is_none")]
    pub any_ue_ind: Option<bool>,
    /// String identifying a group of devices network internal globally unique ID which identifies a set of IMSIs, as specified in clause 19.9 of 3GPP TS 23.003.  
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// Unsigned integer identifying a PDU session, within the range 0 to 255, as specified in  clause 11.2.3.1b, bits 1 to 8, of 3GPP TS 24.007. If the PDU Session ID is allocated by the  Core Network for UEs not supporting N1 mode, reserved range 64 to 95 is used. PDU Session ID  within the reserved range is only visible in the Core Network.  
    #[serde(rename = "pduSeId", skip_serializing_if = "Option::is_none")]
    pub pdu_se_id: Option<i32>,
    /// String representing a Data Network as defined in clause 9A of 3GPP TS 23.003;  it shall contain either a DNN Network Identifier, or a full DNN with both the Network  Identifier and Operator Identifier, as specified in 3GPP TS 23.003 clause 9.1.1 and 9.1.2. It shall be coded as string in which the labels are separated by dots  (e.g. \"Label1.Label2.Label3\").
    #[serde(rename = "dnn", skip_serializing_if = "Option::is_none")]
    pub dnn: Option<String>,
    #[serde(rename = "snssai", skip_serializing_if = "Option::is_none")]
    pub snssai: Option<Box<crate::Snssai>>,
    /// Identifies an Individual SMF Notification Subscription. To enable that the value is used as part of a URI, the string shall only contain characters allowed according to the \"lower-with-hyphen\" naming convention defined in 3GPP TS 29.501. In an OpenAPI schema, the format shall be designated as \"SubId\".
    #[serde(rename = "subId", skip_serializing_if = "Option::is_none")]
    pub sub_id: Option<String>,
    /// Notification Correlation ID assigned by the NF service consumer.
    #[serde(rename = "notifId")]
    pub notif_id: String,
    /// String providing an URI formatted according to RFC 3986.
    #[serde(rename = "notifUri")]
    pub notif_uri: String,
    /// Alternate or backup IPv4 address(es) where to send Notifications.
    #[serde(rename = "altNotifIpv4Addrs", skip_serializing_if = "Option::is_none")]
    pub alt_notif_ipv4_addrs: Option<Vec<String>>,
    /// Alternate or backup IPv6 address(es) where to send Notifications.
    #[serde(rename = "altNotifIpv6Addrs", skip_serializing_if = "Option::is_none")]
    pub alt_notif_ipv6_addrs: Option<Vec<crate::Ipv6Addr>>,
    /// Alternate or backup FQDN(s) where to send Notifications.
    #[serde(rename = "altNotifFqdns", skip_serializing_if = "Option::is_none")]
    pub alt_notif_fqdns: Option<Vec<String>>,
    /// Subscribed events
    #[serde(rename = "eventSubs")]
    pub event_subs: Vec<crate::EventSubscription>,
    #[serde(rename = "eventNotifs", skip_serializing_if = "Option::is_none")]
    pub event_notifs: Option<Vec<crate::EventNotification>>,
    #[serde(rename = "ImmeRep", skip_serializing_if = "Option::is_none")]
    pub imme_rep: Option<bool>,
    #[serde(rename = "notifMethod", skip_serializing_if = "Option::is_none")]
    pub notif_method: Option<Box<crate::NotificationMethod>>,
    /// Unsigned Integer, i.e. only value 0 and integers above 0 are permissible.
    #[serde(rename = "maxReportNbr", skip_serializing_if = "Option::is_none")]
    pub max_report_nbr: Option<i32>,
    /// string with format 'date-time' as defined in OpenAPI.
    #[serde(rename = "expiry", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    /// indicating a time in seconds.
    #[serde(rename = "repPeriod", skip_serializing_if = "Option::is_none")]
    pub rep_period: Option<i32>,
    #[serde(rename = "guami", skip_serializing_if = "Option::is_none")]
    pub guami: Option<Box<crate::Guami>>,
    #[serde(rename = "serviveName", skip_serializing_if = "Option::is_none")]
    pub servive_name: Option<Box<crate::ServiceName>>,
    /// A string used to indicate the features supported by an API that is used as defined in clause  6.6 in 3GPP TS 29.500. The string shall contain a bitmask indicating supported features in  hexadecimal representation Each character in the string shall take a value of \"0\" to \"9\",  \"a\" to \"f\" or \"A\" to \"F\" and shall represent the support of 4 features as described in  table 5.2.2-3. The most significant character representing the highest-numbered features shall  appear first in the string, and the character representing features 1 to 4 shall appear last  in the string. The list of features and their numbering (starting with 1) are defined  separately for each API. If the string contains a lower number of characters than there are  defined features for an API, all features that would be represented by characters that are not  present in the string are not supported.
    #[serde(rename = "supportedFeatures", skip_serializing_if = "Option::is_none")]
    pub supported_features: Option<String>,
    /// Unsigned integer indicating Sampling Ratio (see clauses 4.15.1 of 3GPP TS 23.502), expressed in percent.  
    #[serde(rename = "sampRatio", skip_serializing_if = "Option::is_none")]
    pub samp_ratio: Option<i32>,
    /// Criteria for partitioning the UEs before applying the sampling ratio.
    #[serde(rename = "partitionCriteria", skip_serializing_if = "Option::is_none")]
    pub partition_criteria: Option<Vec<crate::PartitioningCriteria>>,
    /// indicating a time in seconds.
    #[serde(rename = "grpRepTime", skip_serializing_if = "Option::is_none")]
    pub grp_rep_time: Option<i32>,
    #[serde(rename = "notifFlag", skip_serializing_if = "Option::is_none")]
    pub notif_flag: Option<Box<crate::NotificationFlag>>,
}

impl NsmfEventExposure {
    /// Represents an Individual SMF Notification Subscription resource. The serviveName property corresponds to the serviceName in the main body of the specification.
    pub fn new(
        notif_id: String,
        notif_uri: String,
        event_subs: Vec<crate::EventSubscription>,
    ) -> NsmfEventExposure {
        NsmfEventExposure {
            supi: None,
            gpsi: None,
            any_ue_ind: None,
            group_id: None,
            pdu_se_id: None,
            dnn: None,
            snssai: None,
            sub_id: None,
            notif_id,
            notif_uri,
            alt_notif_ipv4_addrs: None,
            alt_notif_ipv6_addrs: None,
            alt_notif_fqdns: None,
            event_subs,
            event_notifs: None,
            imme_rep: None,
            notif_method: None,
            max_report_nbr: None,
            expiry: None,
            rep_period: None,
            guami: None,
            servive_name: None,
            supported_features: None,
            samp_ratio: None,
            partition_criteria: None,
            grp_rep_time: None,
            notif_flag: None,
        }
    }
}
