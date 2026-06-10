pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemPriorityDnsType {
    #[serde(rename = "MX")]
    Mx,
}
impl fmt::Display for DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemPriorityDnsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Mx => "MX",
        };
        write!(f, "{}", s)
    }
}
