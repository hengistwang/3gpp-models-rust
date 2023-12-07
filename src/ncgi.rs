use serde::{Deserialize, Serialize};

/// Ncgi : Contains the NCGI (NR Cell Global Identity), as described in 3GPP 23.003

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ncgi {
    #[serde(rename = "plmnId")]
    pub plmn_id: Box<crate::PlmnId>,
    /// 36-bit string identifying an NR Cell Id as specified in clause 9.3.1.7 of 3GPP TS 38.413,  in hexadecimal representation. Each character in the string shall take a value of \"0\" to \"9\",  \"a\" to \"f\" or \"A\" to \"F\" and shall represent 4 bits. The most significant character  representing the 4 most significant bits of the Cell Id shall appear first in the string, and  the character representing the 4 least significant bit of the Cell Id shall appear last in the  string.  
    #[serde(rename = "nrCellId")]
    pub nr_cell_id: String,
    /// This represents the Network Identifier, which together with a PLMN ID is used to identify an SNPN (see 3GPP TS 23.003 and 3GPP TS 23.501 clause 5.30.2.1).  
    #[serde(rename = "nid", skip_serializing_if = "Option::is_none")]
    pub nid: Option<String>,
}

impl Ncgi {
    /// Contains the NCGI (NR Cell Global Identity), as described in 3GPP 23.003
    pub fn new(plmn_id: crate::PlmnId, nr_cell_id: String) -> Ncgi {
        Ncgi {
            plmn_id: Box::new(plmn_id),
            nr_cell_id,
            nid: None,
        }
    }
}
