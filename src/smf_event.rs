use serde::{Deserialize, Serialize};

/// SmfEvent : Possible values are: - AC_TY_CH: Access Type Change - UP_PATH_CH: UP Path Change - PDU_SES_REL: PDU Session Release - PLMN_CH: PLMN Change - UE_IP_CH: UE IP address change - RAT_TY_CH: RAT Type Change - DDDS: Downlink data delivery status - COMM_FAIL: Communication Failure - PDU_SES_EST: PDU Session Establishment - QFI_ALLOC: QFI allocation - QOS_MON: QoS Monitoring - SMCC_EXP: SM congestion control experience for PDU Session - DISPERSION: Session Management transaction dispersion - RED_TRANS_EXP: Redundant transmission experience for PDU Session - WLAN_INFO: WLAN information on PDU session for which Access Type is NON_3GPP_ACCESS and   RAT Type is TRUSTED_WLAN - UPF_INFO: The UPF information, including the UPF ID/address/FQDN information. - UP_STATUS_INFO: The User Plane status information.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmfEvent {}

impl SmfEvent {
    /// Possible values are: - AC_TY_CH: Access Type Change - UP_PATH_CH: UP Path Change - PDU_SES_REL: PDU Session Release - PLMN_CH: PLMN Change - UE_IP_CH: UE IP address change - RAT_TY_CH: RAT Type Change - DDDS: Downlink data delivery status - COMM_FAIL: Communication Failure - PDU_SES_EST: PDU Session Establishment - QFI_ALLOC: QFI allocation - QOS_MON: QoS Monitoring - SMCC_EXP: SM congestion control experience for PDU Session - DISPERSION: Session Management transaction dispersion - RED_TRANS_EXP: Redundant transmission experience for PDU Session - WLAN_INFO: WLAN information on PDU session for which Access Type is NON_3GPP_ACCESS and   RAT Type is TRUSTED_WLAN - UPF_INFO: The UPF information, including the UPF ID/address/FQDN information. - UP_STATUS_INFO: The User Plane status information.
    pub fn new() -> SmfEvent {
        SmfEvent {}
    }
}
