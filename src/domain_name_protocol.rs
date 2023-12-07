use serde::{Deserialize, Serialize};

/// DomainNameProtocol : Possible values are - DNS_QNAME: Identifies the DNS protocol and the question name in DNS query. - TLS_SNI: Identifies the Server Name Indication in TLS ClientHello message. - TLS_SAN: Identifies the Subject Alternative Name in TLS ServerCertificate message. - TSL_SCN: Identifies the Subject Common Name in TLS ServerCertificate message.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainNameProtocol {}

impl DomainNameProtocol {
    /// Possible values are - DNS_QNAME: Identifies the DNS protocol and the question name in DNS query. - TLS_SNI: Identifies the Server Name Indication in TLS ClientHello message. - TLS_SAN: Identifies the Subject Alternative Name in TLS ServerCertificate message. - TSL_SCN: Identifies the Subject Common Name in TLS ServerCertificate message.
    pub fn new() -> DomainNameProtocol {
        DomainNameProtocol {}
    }
}
