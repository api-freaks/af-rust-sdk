pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DomainDnsLookupResponseDnsRecordsItemSingleNameDnsType {
    #[serde(rename = "NS")]
    Ns,
}
impl fmt::Display for DomainDnsLookupResponseDnsRecordsItemSingleNameDnsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Ns => "NS",
        };
        write!(f, "{}", s)
    }
}
