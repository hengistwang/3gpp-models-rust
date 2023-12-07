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

/// EventNotification : Represents a notification related to a single event that occurred.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventNotification {
    #[serde(rename = "event")]
    pub event: Box<crate::SmfEvent>,
    /// string with format 'date-time' as defined in OpenAPI.
    #[serde(rename = "timeStamp")]
    pub time_stamp: String,
    /// String identifying a Supi that shall contain either an IMSI, a network specific identifier, a Global Cable Identifier (GCI) or a Global Line Identifier (GLI) as specified in clause  2.2A of 3GPP TS 23.003. It shall be formatted as follows  - for an IMSI \"imsi-<imsi>\", where <imsi> shall be formatted according to clause 2.2    of 3GPP TS 23.003 that describes an IMSI.  - for a network specific identifier \"nai-<nai>, where <nai> shall be formatted    according to clause 28.7.2 of 3GPP TS 23.003 that describes an NAI.  - for a GCI \"gci-<gci>\", where <gci> shall be formatted according to clause 28.15.2    of 3GPP TS 23.003.  - for a GLI \"gli-<gli>\", where <gli> shall be formatted according to clause 28.16.2 of    3GPP TS 23.003.To enable that the value is used as part of an URI, the string shall    only contain characters allowed according to the \"lower-with-hyphen\" naming convention    defined in 3GPP TS 29.501.
    #[serde(rename = "supi", skip_serializing_if = "Option::is_none")]
    pub supi: Option<String>,
    /// String identifying a Gpsi shall contain either an External Id or an MSISDN.  It shall be formatted as follows -External Identifier= \"extid-'extid', where 'extid'  shall be formatted according to clause 19.7.2 of 3GPP TS 23.003 that describes an  External Identifier.  
    #[serde(rename = "gpsi", skip_serializing_if = "Option::is_none")]
    pub gpsi: Option<String>,
    #[serde(
        rename = "ueIpAddr",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ue_ip_addr: Option<Option<Box<crate::IpAddr>>>,
    /// Transaction Information.
    #[serde(rename = "transacInfos", skip_serializing_if = "Option::is_none")]
    pub transac_infos: Option<Vec<crate::TransactionInfo>>,
    /// DNAI (Data network access identifier), see clause 5.6.7 of 3GPP TS 23.501.
    #[serde(rename = "sourceDnai", skip_serializing_if = "Option::is_none")]
    pub source_dnai: Option<String>,
    /// DNAI (Data network access identifier), see clause 5.6.7 of 3GPP TS 23.501.
    #[serde(rename = "targetDnai", skip_serializing_if = "Option::is_none")]
    pub target_dnai: Option<String>,
    #[serde(rename = "dnaiChgType", skip_serializing_if = "Option::is_none")]
    pub dnai_chg_type: Option<Box<crate::DnaiChangeType>>,
    /// String identifying a IPv4 address formatted in the 'dotted decimal' notation as defined in RFC 1166.
    #[serde(rename = "sourceUeIpv4Addr", skip_serializing_if = "Option::is_none")]
    pub source_ue_ipv4_addr: Option<String>,
    #[serde(rename = "sourceUeIpv6Prefix", skip_serializing_if = "Option::is_none")]
    pub source_ue_ipv6_prefix: Option<Box<crate::Ipv6Prefix>>,
    /// String identifying a IPv4 address formatted in the 'dotted decimal' notation as defined in RFC 1166.
    #[serde(rename = "targetUeIpv4Addr", skip_serializing_if = "Option::is_none")]
    pub target_ue_ipv4_addr: Option<String>,
    #[serde(rename = "targetUeIpv6Prefix", skip_serializing_if = "Option::is_none")]
    pub target_ue_ipv6_prefix: Option<Box<crate::Ipv6Prefix>>,
    #[serde(
        rename = "sourceTraRouting",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_tra_routing: Option<Option<Box<crate::RouteToLocation>>>,
    #[serde(
        rename = "targetTraRouting",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_tra_routing: Option<Option<Box<crate::RouteToLocation>>>,
    /// String identifying a MAC address formatted in the hexadecimal notation according to clause 1.1 and clause 2.1 of RFC 7042.
    #[serde(rename = "ueMac", skip_serializing_if = "Option::is_none")]
    pub ue_mac: Option<String>,
    /// String identifying a IPv4 address formatted in the 'dotted decimal' notation as defined in RFC 1166.
    #[serde(rename = "adIpv4Addr", skip_serializing_if = "Option::is_none")]
    pub ad_ipv4_addr: Option<String>,
    #[serde(rename = "adIpv6Prefix", skip_serializing_if = "Option::is_none")]
    pub ad_ipv6_prefix: Option<Box<crate::Ipv6Prefix>>,
    /// String identifying a IPv4 address formatted in the 'dotted decimal' notation as defined in RFC 1166.
    #[serde(rename = "reIpv4Addr", skip_serializing_if = "Option::is_none")]
    pub re_ipv4_addr: Option<String>,
    #[serde(rename = "reIpv6Prefix", skip_serializing_if = "Option::is_none")]
    pub re_ipv6_prefix: Option<Box<crate::Ipv6Prefix>>,
    #[serde(rename = "plmnId", skip_serializing_if = "Option::is_none")]
    pub plmn_id: Option<Box<crate::PlmnId>>,
    #[serde(rename = "accType", skip_serializing_if = "Option::is_none")]
    pub acc_type: Option<crate::AccessType>,
    /// Unsigned integer identifying a PDU session, within the range 0 to 255, as specified in  clause 11.2.3.1b, bits 1 to 8, of 3GPP TS 24.007. If the PDU Session ID is allocated by the  Core Network for UEs not supporting N1 mode, reserved range 64 to 95 is used. PDU Session ID  within the reserved range is only visible in the Core Network.  
    #[serde(rename = "pduSeId", skip_serializing_if = "Option::is_none")]
    pub pdu_se_id: Option<i32>,
    #[serde(rename = "ratType", skip_serializing_if = "Option::is_none")]
    pub rat_type: Option<Box<crate::RatType>>,
    #[serde(rename = "dddStatus", skip_serializing_if = "Option::is_none")]
    pub ddd_status: Option<Box<crate::DlDataDeliveryStatus>>,
    #[serde(rename = "dddTraDescriptor", skip_serializing_if = "Option::is_none")]
    pub ddd_tra_descriptor: Option<Box<crate::DddTrafficDescriptor>>,
    /// string with format 'date-time' as defined in OpenAPI.
    #[serde(rename = "maxWaitTime", skip_serializing_if = "Option::is_none")]
    pub max_wait_time: Option<String>,
    #[serde(rename = "commFailure", skip_serializing_if = "Option::is_none")]
    pub comm_failure: Option<Box<crate::CommunicationFailure>>,
    /// String identifying a IPv4 address formatted in the 'dotted decimal' notation as defined in RFC 1166.
    #[serde(rename = "ipv4Addr", skip_serializing_if = "Option::is_none")]
    pub ipv4_addr: Option<String>,
    #[serde(rename = "ipv6Prefixes", skip_serializing_if = "Option::is_none")]
    pub ipv6_prefixes: Option<Vec<crate::Ipv6Prefix>>,
    #[serde(rename = "ipv6Addrs", skip_serializing_if = "Option::is_none")]
    pub ipv6_addrs: Option<Vec<crate::Ipv6Addr>>,
    #[serde(rename = "pduSessType", skip_serializing_if = "Option::is_none")]
    pub pdu_sess_type: Option<Box<crate::PduSessionType>>,
    /// Unsigned integer identifying a QoS flow, within the range 0 to 63.
    #[serde(rename = "qfi", skip_serializing_if = "Option::is_none")]
    pub qfi: Option<i32>,
    /// String providing an application identifier.
    #[serde(rename = "appId", skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// Descriptor(s) for non-IP traffic. It allows the encoding of multiple UL and/or DL flows. Each entry of the array describes a single Ethernet flow.
    #[serde(rename = "ethFlowDescs", skip_serializing_if = "Option::is_none")]
    pub eth_flow_descs: Option<Vec<crate::EthFlowDescription>>,
    /// Contains the UL and/or DL Ethernet flows. Each entry of the array describes a single Ethernet flow.
    #[serde(rename = "ethfDescs", skip_serializing_if = "Option::is_none")]
    pub ethf_descs: Option<Vec<crate::EthFlowDescription>>,
    /// Descriptor(s) for IP traffic. It allows the encoding of multiple UL and/or DL flows. Each entry of the array describes a single IP flow.
    #[serde(rename = "flowDescs", skip_serializing_if = "Option::is_none")]
    pub flow_descs: Option<Vec<String>>,
    /// Contains the UL and/or DL IP flows. Each entry of the array describes a single IP flow.
    #[serde(rename = "fDescs", skip_serializing_if = "Option::is_none")]
    pub f_descs: Option<Vec<String>>,
    /// String representing a Data Network as defined in clause 9A of 3GPP TS 23.003;  it shall contain either a DNN Network Identifier, or a full DNN with both the Network  Identifier and Operator Identifier, as specified in 3GPP TS 23.003 clause 9.1.1 and 9.1.2. It shall be coded as string in which the labels are separated by dots  (e.g. \"Label1.Label2.Label3\").
    #[serde(rename = "dnn", skip_serializing_if = "Option::is_none")]
    pub dnn: Option<String>,
    #[serde(rename = "snssai", skip_serializing_if = "Option::is_none")]
    pub snssai: Option<Box<crate::Snssai>>,
    #[serde(rename = "ulDelays", skip_serializing_if = "Option::is_none")]
    pub ul_delays: Option<Vec<i32>>,
    #[serde(rename = "dlDelays", skip_serializing_if = "Option::is_none")]
    pub dl_delays: Option<Vec<i32>>,
    #[serde(rename = "rtDelays", skip_serializing_if = "Option::is_none")]
    pub rt_delays: Option<Vec<i32>>,
    /// Represents the packet delay measurement failure indicator.
    #[serde(rename = "pdmf", skip_serializing_if = "Option::is_none")]
    pub pdmf: Option<bool>,
    #[serde(rename = "timeWindow", skip_serializing_if = "Option::is_none")]
    pub time_window: Option<Box<crate::TimeWindow>>,
    #[serde(rename = "smNasFromUe", skip_serializing_if = "Option::is_none")]
    pub sm_nas_from_ue: Option<Box<crate::SmNasFromUe>>,
    #[serde(rename = "smNasFromSmf", skip_serializing_if = "Option::is_none")]
    pub sm_nas_from_smf: Option<Box<crate::SmNasFromSmf>>,
    /// Indicates whether the redundant transmission is setup or terminated. Set to \"true\" if  the redundant transmission is setup, otherwise set to \"false\" if the redundant  transmission is terminated. Default value is set to \"false\".
    #[serde(rename = "upRedTrans", skip_serializing_if = "Option::is_none")]
    pub up_red_trans: Option<bool>,
    #[serde(rename = "ssId", skip_serializing_if = "Option::is_none")]
    pub ss_id: Option<String>,
    #[serde(rename = "bssId", skip_serializing_if = "Option::is_none")]
    pub bss_id: Option<String>,
    /// string with format 'date-time' as defined in OpenAPI.
    #[serde(rename = "startWlan", skip_serializing_if = "Option::is_none")]
    pub start_wlan: Option<String>,
    /// string with format 'date-time' as defined in OpenAPI.
    #[serde(rename = "endWlan", skip_serializing_if = "Option::is_none")]
    pub end_wlan: Option<String>,
    #[serde(rename = "pduSessInfos", skip_serializing_if = "Option::is_none")]
    pub pdu_sess_infos: Option<Vec<crate::PduSessionInformation>>,
    #[serde(rename = "upfInfo", skip_serializing_if = "Option::is_none")]
    pub upf_info: Option<Box<crate::UpfInformation>>,
}

impl EventNotification {
    /// Represents a notification related to a single event that occurred.
    pub fn new(event: crate::SmfEvent, time_stamp: String) -> EventNotification {
        EventNotification {
            event: Box::new(event),
            time_stamp,
            supi: None,
            gpsi: None,
            ue_ip_addr: None,
            transac_infos: None,
            source_dnai: None,
            target_dnai: None,
            dnai_chg_type: None,
            source_ue_ipv4_addr: None,
            source_ue_ipv6_prefix: None,
            target_ue_ipv4_addr: None,
            target_ue_ipv6_prefix: None,
            source_tra_routing: None,
            target_tra_routing: None,
            ue_mac: None,
            ad_ipv4_addr: None,
            ad_ipv6_prefix: None,
            re_ipv4_addr: None,
            re_ipv6_prefix: None,
            plmn_id: None,
            acc_type: None,
            pdu_se_id: None,
            rat_type: None,
            ddd_status: None,
            ddd_tra_descriptor: None,
            max_wait_time: None,
            comm_failure: None,
            ipv4_addr: None,
            ipv6_prefixes: None,
            ipv6_addrs: None,
            pdu_sess_type: None,
            qfi: None,
            app_id: None,
            eth_flow_descs: None,
            ethf_descs: None,
            flow_descs: None,
            f_descs: None,
            dnn: None,
            snssai: None,
            ul_delays: None,
            dl_delays: None,
            rt_delays: None,
            pdmf: None,
            time_window: None,
            sm_nas_from_ue: None,
            sm_nas_from_smf: None,
            up_red_trans: None,
            ss_id: None,
            bss_id: None,
            start_wlan: None,
            end_wlan: None,
            pdu_sess_infos: None,
            upf_info: None,
        }
    }
}
