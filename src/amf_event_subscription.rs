use serde::{Deserialize, Serialize};

/// AmfEventSubscription : Represents an individual event subscription resource on AMF

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmfEventSubscription {
    #[serde(rename = "eventList")]
    pub event_list: Vec<crate::AmfEvent>,
    /// String providing an URI formatted according to RFC 3986.
    #[serde(rename = "eventNotifyUri")]
    pub event_notify_uri: String,
    #[serde(rename = "notifyCorrelationId")]
    pub notify_correlation_id: String,
    #[serde(rename = "nfId")]
    pub nf_id: uuid::Uuid,
    /// String providing an URI formatted according to RFC 3986.
    #[serde(
        rename = "subsChangeNotifyUri",
        skip_serializing_if = "Option::is_none"
    )]
    pub subs_change_notify_uri: Option<String>,
    #[serde(
        rename = "subsChangeNotifyCorrelationId",
        skip_serializing_if = "Option::is_none"
    )]
    pub subs_change_notify_correlation_id: Option<String>,
    /// String identifying a Supi that shall contain either an IMSI, a network specific identifier, a Global Cable Identifier (GCI) or a Global Line Identifier (GLI) as specified in clause  2.2A of 3GPP TS 23.003. It shall be formatted as follows  - for an IMSI \"imsi-<imsi>\", where <imsi> shall be formatted according to clause 2.2    of 3GPP TS 23.003 that describes an IMSI.  - for a network specific identifier \"nai-<nai>, where <nai> shall be formatted    according to clause 28.7.2 of 3GPP TS 23.003 that describes an NAI.  - for a GCI \"gci-<gci>\", where <gci> shall be formatted according to clause 28.15.2    of 3GPP TS 23.003.  - for a GLI \"gli-<gli>\", where <gli> shall be formatted according to clause 28.16.2 of    3GPP TS 23.003.To enable that the value is used as part of an URI, the string shall    only contain characters allowed according to the \"lower-with-hyphen\" naming convention    defined in 3GPP TS 29.501.
    #[serde(rename = "supi", skip_serializing_if = "Option::is_none")]
    pub supi: Option<String>,
    /// String identifying a group of devices network internal globally unique ID which identifies a set of IMSIs, as specified in clause 19.9 of 3GPP TS 23.003.  
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "excludeSupiList", skip_serializing_if = "Option::is_none")]
    pub exclude_supi_list: Option<Vec<String>>,
    #[serde(rename = "excludeGpsiList", skip_serializing_if = "Option::is_none")]
    pub exclude_gpsi_list: Option<Vec<String>>,
    #[serde(rename = "includeSupiList", skip_serializing_if = "Option::is_none")]
    pub include_supi_list: Option<Vec<String>>,
    #[serde(rename = "includeGpsiList", skip_serializing_if = "Option::is_none")]
    pub include_gpsi_list: Option<Vec<String>>,
    /// String identifying a Gpsi shall contain either an External Id or an MSISDN.  It shall be formatted as follows -External Identifier= \"extid-'extid', where 'extid'  shall be formatted according to clause 19.7.2 of 3GPP TS 23.003 that describes an  External Identifier.  
    #[serde(rename = "gpsi", skip_serializing_if = "Option::is_none")]
    pub gpsi: Option<String>,
    /// String representing a Permanent Equipment Identifier that may contain - an IMEI or IMEISV, as  specified in clause 6.2 of 3GPP TS 23.003; a MAC address for a 5G-RG or FN-RG via  wireline  access, with an indication that this address cannot be trusted for regulatory purpose if this  address cannot be used as an Equipment Identifier of the FN-RG, as specified in clause 4.7.7  of 3GPP TS23.316. Examples are imei-012345678901234 or imeisv-0123456789012345.  
    #[serde(rename = "pei", skip_serializing_if = "Option::is_none")]
    pub pei: Option<String>,
    #[serde(rename = "anyUE", skip_serializing_if = "Option::is_none")]
    pub any_ue: Option<bool>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<crate::AmfEventMode>>,
    #[serde(rename = "sourceNfType", skip_serializing_if = "Option::is_none")]
    pub source_nf_type: Option<Box<crate::NfType>>,
}

impl AmfEventSubscription {
    /// Represents an individual event subscription resource on AMF
    pub fn new(
        event_list: Vec<crate::AmfEvent>,
        event_notify_uri: String,
        notify_correlation_id: String,
        nf_id: uuid::Uuid,
    ) -> AmfEventSubscription {
        AmfEventSubscription {
            event_list,
            event_notify_uri,
            notify_correlation_id,
            nf_id,
            subs_change_notify_uri: None,
            subs_change_notify_correlation_id: None,
            supi: None,
            group_id: None,
            exclude_supi_list: None,
            exclude_gpsi_list: None,
            include_supi_list: None,
            include_gpsi_list: None,
            gpsi: None,
            pei: None,
            any_ue: None,
            options: None,
            source_nf_type: None,
        }
    }
}
