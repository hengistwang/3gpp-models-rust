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

/// DlDataDeliveryStatus : Possible values are: - BUFFERED: The first downlink data is buffered with extended buffering matching the   source of the downlink traffic. - TRANSMITTED: The first downlink data matching the source of the downlink traffic is   transmitted after previous buffering or discarding of corresponding packet(s) because   the UE of the PDU Session becomes ACTIVE, and buffered data can be delivered to UE. - DISCARDED: The first downlink data matching the source of the downlink traffic is   discarded because the Extended Buffering time, as determined by the SMF, expires or   the amount of downlink data to be buffered is exceeded.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DlDataDeliveryStatus {}

impl DlDataDeliveryStatus {
    /// Possible values are: - BUFFERED: The first downlink data is buffered with extended buffering matching the   source of the downlink traffic. - TRANSMITTED: The first downlink data matching the source of the downlink traffic is   transmitted after previous buffering or discarding of corresponding packet(s) because   the UE of the PDU Session becomes ACTIVE, and buffered data can be delivered to UE. - DISCARDED: The first downlink data matching the source of the downlink traffic is   discarded because the Extended Buffering time, as determined by the SMF, expires or   the amount of downlink data to be buffered is exceeded.
    pub fn new() -> DlDataDeliveryStatus {
        DlDataDeliveryStatus {}
    }
}
