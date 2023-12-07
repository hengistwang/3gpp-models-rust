use serde::{Deserialize, Serialize};

/// AckOfNotify : Represents an acknowledgement information of an event notification.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AckOfNotify {
    #[serde(rename = "notifId")]
    pub notif_id: String,
    #[serde(rename = "ackResult")]
    pub ack_result: Box<crate::AfResultInfo>,
    /// String identifying a Supi that shall contain either an IMSI, a network specific identifier, a Global Cable Identifier (GCI) or a Global Line Identifier (GLI) as specified in clause  2.2A of 3GPP TS 23.003. It shall be formatted as follows  - for an IMSI \"imsi-<imsi>\", where <imsi> shall be formatted according to clause 2.2    of 3GPP TS 23.003 that describes an IMSI.  - for a network specific identifier \"nai-<nai>, where <nai> shall be formatted    according to clause 28.7.2 of 3GPP TS 23.003 that describes an NAI.  - for a GCI \"gci-<gci>\", where <gci> shall be formatted according to clause 28.15.2    of 3GPP TS 23.003.  - for a GLI \"gli-<gli>\", where <gli> shall be formatted according to clause 28.16.2 of    3GPP TS 23.003.To enable that the value is used as part of an URI, the string shall    only contain characters allowed according to the \"lower-with-hyphen\" naming convention    defined in 3GPP TS 29.501.
    #[serde(rename = "supi", skip_serializing_if = "Option::is_none")]
    pub supi: Option<String>,
    /// String identifying a Gpsi shall contain either an External Id or an MSISDN.  It shall be formatted as follows -External Identifier= \"extid-'extid', where 'extid'  shall be formatted according to clause 19.7.2 of 3GPP TS 23.003 that describes an  External Identifier.  
    #[serde(rename = "gpsi", skip_serializing_if = "Option::is_none")]
    pub gpsi: Option<String>,
}

impl AckOfNotify {
    /// Represents an acknowledgement information of an event notification.
    pub fn new(notif_id: String, ack_result: crate::AfResultInfo) -> AckOfNotify {
        AckOfNotify {
            notif_id,
            ack_result: Box::new(ack_result),
            supi: None,
            gpsi: None,
        }
    }
}
