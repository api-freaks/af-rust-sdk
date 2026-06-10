pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemOneDnsType {
    #[serde(rename = "CNAME")]
    Cname,
}
impl fmt::Display for BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemOneDnsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Cname => "CNAME",
        };
        write!(f, "{}", s)
    }
}
