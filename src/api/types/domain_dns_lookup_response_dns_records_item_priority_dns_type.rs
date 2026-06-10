pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DomainDnsLookupResponseDnsRecordsItemPriorityDnsType {
    #[serde(rename = "MX")]
    Mx,
}
impl fmt::Display for DomainDnsLookupResponseDnsRecordsItemPriorityDnsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Mx => "MX",
        };
        write!(f, "{}", s)
    }
}
