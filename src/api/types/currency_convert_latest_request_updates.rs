pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CurrencyConvertLatestRequestUpdates {
    OneD,
    OneH,
    TenM,
    OneM,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CurrencyConvertLatestRequestUpdates {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::OneD => serializer.serialize_str("1d"),
            Self::OneH => serializer.serialize_str("1h"),
            Self::TenM => serializer.serialize_str("10m"),
            Self::OneM => serializer.serialize_str("1m"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CurrencyConvertLatestRequestUpdates {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "1d" => Ok(Self::OneD),
            "1h" => Ok(Self::OneH),
            "10m" => Ok(Self::TenM),
            "1m" => Ok(Self::OneM),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CurrencyConvertLatestRequestUpdates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OneD => write!(f, "1d"),
            Self::OneH => write!(f, "1h"),
            Self::TenM => write!(f, "10m"),
            Self::OneM => write!(f, "1m"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
