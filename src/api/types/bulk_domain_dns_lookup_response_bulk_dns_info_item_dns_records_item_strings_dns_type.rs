pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStringsDnsType {
    Txt,
    Spf,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStringsDnsType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Txt => serializer.serialize_str("TXT"),
            Self::Spf => serializer.serialize_str("SPF"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de>
    for BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStringsDnsType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "TXT" => Ok(Self::Txt),
            "SPF" => Ok(Self::Spf),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStringsDnsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Txt => write!(f, "TXT"),
            Self::Spf => write!(f, "SPF"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
