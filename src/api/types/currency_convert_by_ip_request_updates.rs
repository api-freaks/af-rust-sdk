pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CurrencyConvertByIpRequestUpdates {
    OneM,
    TenM,
    OneH,
    OneD,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CurrencyConvertByIpRequestUpdates {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::OneM => serializer.serialize_str("1m"),
            Self::TenM => serializer.serialize_str("10m"),
            Self::OneH => serializer.serialize_str("1h"),
            Self::OneD => serializer.serialize_str("1d"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CurrencyConvertByIpRequestUpdates {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "1m" => Ok(Self::OneM),
            "10m" => Ok(Self::TenM),
            "1h" => Ok(Self::OneH),
            "1d" => Ok(Self::OneD),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CurrencyConvertByIpRequestUpdates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OneM => write!(f, "1m"),
            Self::TenM => write!(f, "10m"),
            Self::OneH => write!(f, "1h"),
            Self::OneD => write!(f, "1d"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
