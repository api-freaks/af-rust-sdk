pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOneDnsType {
    #[serde(rename = "CNAME")]
    Cname,
}
impl fmt::Display for DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOneDnsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Cname => "CNAME",
        };
        write!(f, "{}", s)
    }
}
