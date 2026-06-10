pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PdfCompressRequestCompressionLevel {
    Low,
    Balanced,
    High,
    Extreme,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PdfCompressRequestCompressionLevel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Low => serializer.serialize_str("low"),
            Self::Balanced => serializer.serialize_str("balanced"),
            Self::High => serializer.serialize_str("high"),
            Self::Extreme => serializer.serialize_str("extreme"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PdfCompressRequestCompressionLevel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "low" => Ok(Self::Low),
            "balanced" => Ok(Self::Balanced),
            "high" => Ok(Self::High),
            "extreme" => Ok(Self::Extreme),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PdfCompressRequestCompressionLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Low => write!(f, "low"),
            Self::Balanced => write!(f, "balanced"),
            Self::High => write!(f, "high"),
            Self::Extreme => write!(f, "extreme"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
