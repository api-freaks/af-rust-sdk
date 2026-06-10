pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DomainDnsReverseRequestType {
    A,
    Aaaa,
    Mx,
    Ns,
    Soa,
    Spf,
    Txt,
    Cname,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DomainDnsReverseRequestType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::A => serializer.serialize_str("A"),
            Self::Aaaa => serializer.serialize_str("AAAA"),
            Self::Mx => serializer.serialize_str("MX"),
            Self::Ns => serializer.serialize_str("NS"),
            Self::Soa => serializer.serialize_str("SOA"),
            Self::Spf => serializer.serialize_str("SPF"),
            Self::Txt => serializer.serialize_str("TXT"),
            Self::Cname => serializer.serialize_str("CNAME"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DomainDnsReverseRequestType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "A" => Ok(Self::A),
            "AAAA" => Ok(Self::Aaaa),
            "MX" => Ok(Self::Mx),
            "NS" => Ok(Self::Ns),
            "SOA" => Ok(Self::Soa),
            "SPF" => Ok(Self::Spf),
            "TXT" => Ok(Self::Txt),
            "CNAME" => Ok(Self::Cname),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DomainDnsReverseRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::Aaaa => write!(f, "AAAA"),
            Self::Mx => write!(f, "MX"),
            Self::Ns => write!(f, "NS"),
            Self::Soa => write!(f, "SOA"),
            Self::Spf => write!(f, "SPF"),
            Self::Txt => write!(f, "TXT"),
            Self::Cname => write!(f, "CNAME"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
