pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DomainDnsLookupResponseDnsRecordsItemAdminDnsType {
    #[serde(rename = "SOA")]
    Soa,
}
impl fmt::Display for DomainDnsLookupResponseDnsRecordsItemAdminDnsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Soa => "SOA",
        };
        write!(f, "{}", s)
    }
}
