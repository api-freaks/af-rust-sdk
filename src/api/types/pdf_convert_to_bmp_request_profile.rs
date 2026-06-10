pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PdfConvertToBmpRequestProfile {
    Bw,
    Gray,
    Rgb,
    FourBit,
    EightBit,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PdfConvertToBmpRequestProfile {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Bw => serializer.serialize_str("bw"),
            Self::Gray => serializer.serialize_str("gray"),
            Self::Rgb => serializer.serialize_str("rgb"),
            Self::FourBit => serializer.serialize_str("4-bit"),
            Self::EightBit => serializer.serialize_str("8-bit"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PdfConvertToBmpRequestProfile {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "bw" => Ok(Self::Bw),
            "gray" => Ok(Self::Gray),
            "rgb" => Ok(Self::Rgb),
            "4-bit" => Ok(Self::FourBit),
            "8-bit" => Ok(Self::EightBit),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PdfConvertToBmpRequestProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bw => write!(f, "bw"),
            Self::Gray => write!(f, "gray"),
            Self::Rgb => write!(f, "rgb"),
            Self::FourBit => write!(f, "4-bit"),
            Self::EightBit => write!(f, "8-bit"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
