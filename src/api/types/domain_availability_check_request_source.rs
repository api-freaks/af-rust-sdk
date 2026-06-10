pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DomainAvailabilityCheckRequestSource {
    Dns,
    Whois,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DomainAvailabilityCheckRequestSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Dns => serializer.serialize_str("dns"),
            Self::Whois => serializer.serialize_str("whois"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DomainAvailabilityCheckRequestSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "dns" => Ok(Self::Dns),
            "whois" => Ok(Self::Whois),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DomainAvailabilityCheckRequestSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dns => write!(f, "dns"),
            Self::Whois => write!(f, "whois"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
