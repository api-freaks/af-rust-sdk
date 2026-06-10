pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetFlagsRequestShape {
    Flat,
    Round,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetFlagsRequestShape {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Flat => serializer.serialize_str("flat"),
            Self::Round => serializer.serialize_str("round"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetFlagsRequestShape {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "flat" => Ok(Self::Flat),
            "round" => Ok(Self::Round),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetFlagsRequestShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Flat => write!(f, "flat"),
            Self::Round => write!(f, "round"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
