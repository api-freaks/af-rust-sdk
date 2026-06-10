pub use crate::prelude::*;

/// The rate at which this commodity's price is updated.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommoditySymbolsResponseSymbolsItemUpdateInterval {
    PerSecond,
    PerMinute,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CommoditySymbolsResponseSymbolsItemUpdateInterval {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::PerSecond => serializer.serialize_str("PER_SECOND"),
            Self::PerMinute => serializer.serialize_str("PER_MINUTE"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CommoditySymbolsResponseSymbolsItemUpdateInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "PER_SECOND" => Ok(Self::PerSecond),
            "PER_MINUTE" => Ok(Self::PerMinute),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CommoditySymbolsResponseSymbolsItemUpdateInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PerSecond => write!(f, "PER_SECOND"),
            Self::PerMinute => write!(f, "PER_MINUTE"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
