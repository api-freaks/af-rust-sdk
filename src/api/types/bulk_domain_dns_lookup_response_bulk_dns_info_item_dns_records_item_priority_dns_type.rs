pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemPriorityDnsType {
    #[serde(rename = "MX")]
    Mx,
}
impl fmt::Display for BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemPriorityDnsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Mx => "MX",
        };
        write!(f, "{}", s)
    }
}
