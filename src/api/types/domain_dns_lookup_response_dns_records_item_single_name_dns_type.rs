pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DomainDnsLookupResponseDnsRecordsItemSingleNameDnsType {
    Ns,
    Ptr,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DomainDnsLookupResponseDnsRecordsItemSingleNameDnsType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ns => serializer.serialize_str("NS"),
            Self::Ptr => serializer.serialize_str("PTR"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DomainDnsLookupResponseDnsRecordsItemSingleNameDnsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "NS" => Ok(Self::Ns),
            "PTR" => Ok(Self::Ptr),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DomainDnsLookupResponseDnsRecordsItemSingleNameDnsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ns => write!(f, "NS"),
            Self::Ptr => write!(f, "PTR"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
