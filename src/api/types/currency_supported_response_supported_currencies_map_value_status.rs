pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CurrencySupportedResponseSupportedCurrenciesMapValueStatus {
    Available,
    Depreciated,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CurrencySupportedResponseSupportedCurrenciesMapValueStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Available => serializer.serialize_str("AVAILABLE"),
            Self::Depreciated => serializer.serialize_str("DEPRECIATED"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CurrencySupportedResponseSupportedCurrenciesMapValueStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "AVAILABLE" => Ok(Self::Available),
            "DEPRECIATED" => Ok(Self::Depreciated),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CurrencySupportedResponseSupportedCurrenciesMapValueStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Available => write!(f, "AVAILABLE"),
            Self::Depreciated => write!(f, "DEPRECIATED"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
