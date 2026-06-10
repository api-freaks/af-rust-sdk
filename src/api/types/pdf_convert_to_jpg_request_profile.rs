pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PdfConvertToJpgRequestProfile {
    Gray,
    Rgb,
    Cmyk,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PdfConvertToJpgRequestProfile {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Gray => serializer.serialize_str("gray"),
            Self::Rgb => serializer.serialize_str("rgb"),
            Self::Cmyk => serializer.serialize_str("cmyk"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PdfConvertToJpgRequestProfile {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "gray" => Ok(Self::Gray),
            "rgb" => Ok(Self::Rgb),
            "cmyk" => Ok(Self::Cmyk),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PdfConvertToJpgRequestProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gray => write!(f, "gray"),
            Self::Rgb => write!(f, "rgb"),
            Self::Cmyk => write!(f, "cmyk"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
