use serde::{Deserialize, Serialize};

/// AfResultInfo : Identifies the result of application layer handling.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AfResultInfo {
    #[serde(rename = "afStatus")]
    pub af_status: Box<crate::AfResultStatus>,
    #[serde(
        rename = "trafficRoute",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_route: Option<Option<Box<crate::RouteToLocation>>>,
    /// If present and set to \"true\" it indicates that buffering of uplink traffic to the target DNAI is needed.
    #[serde(rename = "upBuffInd", skip_serializing_if = "Option::is_none")]
    pub up_buff_ind: Option<bool>,
    /// Contains EAS IP replacement information.
    #[serde(rename = "easIpReplaceInfos", skip_serializing_if = "Option::is_none")]
    pub eas_ip_replace_infos: Option<Vec<crate::EasIpReplacementInfo>>,
}

impl AfResultInfo {
    /// Identifies the result of application layer handling.
    pub fn new(af_status: crate::AfResultStatus) -> AfResultInfo {
        AfResultInfo {
            af_status: Box::new(af_status),
            traffic_route: None,
            up_buff_ind: None,
            eas_ip_replace_infos: None,
        }
    }
}
