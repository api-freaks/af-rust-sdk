pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ZipcodeSearchByRadiusRequestUnit {
    M,
    Km,
    Mi,
    Ft,
    Yd,
    In,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ZipcodeSearchByRadiusRequestUnit {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::M => serializer.serialize_str("m"),
            Self::Km => serializer.serialize_str("km"),
            Self::Mi => serializer.serialize_str("mi"),
            Self::Ft => serializer.serialize_str("ft"),
            Self::Yd => serializer.serialize_str("yd"),
            Self::In => serializer.serialize_str("in"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ZipcodeSearchByRadiusRequestUnit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "m" => Ok(Self::M),
            "km" => Ok(Self::Km),
            "mi" => Ok(Self::Mi),
            "ft" => Ok(Self::Ft),
            "yd" => Ok(Self::Yd),
            "in" => Ok(Self::In),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ZipcodeSearchByRadiusRequestUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::M => write!(f, "m"),
            Self::Km => write!(f, "km"),
            Self::Mi => write!(f, "mi"),
            Self::Ft => write!(f, "ft"),
            Self::Yd => write!(f, "yd"),
            Self::In => write!(f, "in"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
