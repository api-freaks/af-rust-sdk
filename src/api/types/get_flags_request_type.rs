pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetFlagsRequestType {
    Country,
    Organization,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetFlagsRequestType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Country => serializer.serialize_str("country"),
            Self::Organization => serializer.serialize_str("organization"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetFlagsRequestType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "country" => Ok(Self::Country),
            "organization" => Ok(Self::Organization),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetFlagsRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Country => write!(f, "country"),
            Self::Organization => write!(f, "organization"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
