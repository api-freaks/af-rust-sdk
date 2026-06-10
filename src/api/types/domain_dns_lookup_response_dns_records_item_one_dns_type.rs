pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DomainDnsLookupResponseDnsRecordsItemOneDnsType {
    #[serde(rename = "CNAME")]
    Cname,
}
impl fmt::Display for DomainDnsLookupResponseDnsRecordsItemOneDnsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Cname => "CNAME",
        };
        write!(f, "{}", s)
    }
}
