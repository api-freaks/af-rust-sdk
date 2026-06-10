pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemOneDnsType {
    #[serde(rename = "CNAME")]
    Cname,
}
impl fmt::Display for DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemOneDnsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Cname => "CNAME",
        };
        write!(f, "{}", s)
    }
}
