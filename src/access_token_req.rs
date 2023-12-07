use serde::{Deserialize, Serialize};

/// AccessTokenReq : Contains information related to the access token request

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessTokenReq {
    #[serde(rename = "grant_type")]
    pub grant_type: GrantType,
    #[serde(rename = "nfInstanceId")]
    pub nf_instance_id: uuid::Uuid,
    #[serde(rename = "nfType", skip_serializing_if = "Option::is_none")]
    pub nf_type: Option<Box<crate::NfType>>,
    #[serde(rename = "targetNfType", skip_serializing_if = "Option::is_none")]
    pub target_nf_type: Option<Box<crate::NfType>>,
    #[serde(rename = "scope")]
    pub scope: String,
    /// String uniquely identifying a NF instance. The format of the NF Instance ID shall be a  Universally Unique Identifier (UUID) version 4, as described in IETF RFC 4122.  
    #[serde(rename = "targetNfInstanceId", skip_serializing_if = "Option::is_none")]
    pub target_nf_instance_id: Option<uuid::Uuid>,
    #[serde(rename = "requesterPlmn", skip_serializing_if = "Option::is_none")]
    pub requester_plmn: Option<Box<crate::PlmnId>>,
    #[serde(rename = "requesterPlmnList", skip_serializing_if = "Option::is_none")]
    pub requester_plmn_list: Option<Vec<crate::PlmnId>>,
    #[serde(
        rename = "requesterSnssaiList",
        skip_serializing_if = "Option::is_none"
    )]
    pub requester_snssai_list: Option<Vec<crate::Snssai>>,
    /// Fully Qualified Domain Name
    #[serde(rename = "requesterFqdn", skip_serializing_if = "Option::is_none")]
    pub requester_fqdn: Option<String>,
    #[serde(rename = "requesterSnpnList", skip_serializing_if = "Option::is_none")]
    pub requester_snpn_list: Option<Vec<crate::PlmnIdNid>>,
    #[serde(rename = "targetPlmn", skip_serializing_if = "Option::is_none")]
    pub target_plmn: Option<Box<crate::PlmnId>>,
    #[serde(rename = "targetSnpn", skip_serializing_if = "Option::is_none")]
    pub target_snpn: Option<Box<crate::PlmnIdNid>>,
    #[serde(rename = "targetSnssaiList", skip_serializing_if = "Option::is_none")]
    pub target_snssai_list: Option<Vec<crate::Snssai>>,
    #[serde(rename = "targetNsiList", skip_serializing_if = "Option::is_none")]
    pub target_nsi_list: Option<Vec<String>>,
    /// NF Set Identifier (see clause 28.12 of 3GPP TS 23.003), formatted as the following string \"set<Set ID>.<nftype>set.5gc.mnc<MNC>.mcc<MCC>\", or  \"set<SetID>.<NFType>set.5gc.nid<NID>.mnc<MNC>.mcc<MCC>\" with  <MCC> encoded as defined in clause 5.4.2 (\"Mcc\" data type definition)  <MNC> encoding the Mobile Network Code part of the PLMN, comprising 3 digits.    If there are only 2 significant digits in the MNC, one \"0\" digit shall be inserted    at the left side to fill the 3 digits coding of MNC.  Pattern: '^[0-9]{3}$' <NFType> encoded as a value defined in Table 6.1.6.3.3-1 of 3GPP TS 29.510 but    with lower case characters <Set ID> encoded as a string of characters consisting of    alphabetic characters (A-Z and a-z), digits (0-9) and/or the hyphen (-) and that    shall end with either an alphabetic character or a digit.  
    #[serde(rename = "targetNfSetId", skip_serializing_if = "Option::is_none")]
    pub target_nf_set_id: Option<String>,
    /// NF Service Set Identifier (see clause 28.12 of 3GPP TS 23.003) formatted as the following  string \"set<Set ID>.sn<Service Name>.nfi<NF Instance ID>.5gc.mnc<MNC>.mcc<MCC>\", or  \"set<SetID>.sn<ServiceName>.nfi<NFInstanceID>.5gc.nid<NID>.mnc<MNC>.mcc<MCC>\" with  <MCC> encoded as defined in clause 5.4.2 (\"Mcc\" data type definition)   <MNC> encoding the Mobile Network Code part of the PLMN, comprising 3 digits.    If there are only 2 significant digits in the MNC, one \"0\" digit shall be inserted    at the left side to fill the 3 digits coding of MNC.  Pattern: '^[0-9]{3}$' <NID> encoded as defined in clause 5.4.2 (\"Nid\" data type definition)  <NFInstanceId> encoded as defined in clause 5.3.2  <ServiceName> encoded as defined in 3GPP TS 29.510  <Set ID> encoded as a string of characters consisting of alphabetic    characters (A-Z and a-z), digits (0-9) and/or the hyphen (-) and that shall end    with either an alphabetic character or a digit.
    #[serde(
        rename = "targetNfServiceSetId",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_nf_service_set_id: Option<String>,
    /// String providing an URI formatted according to RFC 3986.
    #[serde(rename = "hnrfAccessTokenUri", skip_serializing_if = "Option::is_none")]
    pub hnrf_access_token_uri: Option<String>,
    /// String uniquely identifying a NF instance. The format of the NF Instance ID shall be a  Universally Unique Identifier (UUID) version 4, as described in IETF RFC 4122.  
    #[serde(rename = "sourceNfInstanceId", skip_serializing_if = "Option::is_none")]
    pub source_nf_instance_id: Option<uuid::Uuid>,
}

impl AccessTokenReq {
    /// Contains information related to the access token request
    pub fn new(grant_type: GrantType, nf_instance_id: uuid::Uuid, scope: String) -> AccessTokenReq {
        AccessTokenReq {
            grant_type,
            nf_instance_id,
            nf_type: None,
            target_nf_type: None,
            scope,
            target_nf_instance_id: None,
            requester_plmn: None,
            requester_plmn_list: None,
            requester_snssai_list: None,
            requester_fqdn: None,
            requester_snpn_list: None,
            target_plmn: None,
            target_snpn: None,
            target_snssai_list: None,
            target_nsi_list: None,
            target_nf_set_id: None,
            target_nf_service_set_id: None,
            hnrf_access_token_uri: None,
            source_nf_instance_id: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GrantType {
    #[serde(rename = "client_credentials")]
    ClientCredentials,
}

impl Default for GrantType {
    fn default() -> GrantType {
        Self::ClientCredentials
    }
}
