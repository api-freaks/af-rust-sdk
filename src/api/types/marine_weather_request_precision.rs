pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MarineWeatherRequestPrecision {
    Daily,
    Hourly,
    Minutely,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MarineWeatherRequestPrecision {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Daily => serializer.serialize_str("daily"),
            Self::Hourly => serializer.serialize_str("hourly"),
            Self::Minutely => serializer.serialize_str("minutely"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MarineWeatherRequestPrecision {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "daily" => Ok(Self::Daily),
            "hourly" => Ok(Self::Hourly),
            "minutely" => Ok(Self::Minutely),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MarineWeatherRequestPrecision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Daily => write!(f, "daily"),
            Self::Hourly => write!(f, "hourly"),
            Self::Minutely => write!(f, "minutely"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
