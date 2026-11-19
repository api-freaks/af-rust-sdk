pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScreenshotCaptureRequestPdfOptionsFormat {
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    Letter,
    Legal,
    Tabloid,
    Ledger,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ScreenshotCaptureRequestPdfOptionsFormat {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::A0 => serializer.serialize_str("A0"),
            Self::A1 => serializer.serialize_str("A1"),
            Self::A2 => serializer.serialize_str("A2"),
            Self::A3 => serializer.serialize_str("A3"),
            Self::A4 => serializer.serialize_str("A4"),
            Self::A5 => serializer.serialize_str("A5"),
            Self::A6 => serializer.serialize_str("A6"),
            Self::Letter => serializer.serialize_str("Letter"),
            Self::Legal => serializer.serialize_str("Legal"),
            Self::Tabloid => serializer.serialize_str("Tabloid"),
            Self::Ledger => serializer.serialize_str("Ledger"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ScreenshotCaptureRequestPdfOptionsFormat {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "A0" => Ok(Self::A0),
            "A1" => Ok(Self::A1),
            "A2" => Ok(Self::A2),
            "A3" => Ok(Self::A3),
            "A4" => Ok(Self::A4),
            "A5" => Ok(Self::A5),
            "A6" => Ok(Self::A6),
            "Letter" => Ok(Self::Letter),
            "Legal" => Ok(Self::Legal),
            "Tabloid" => Ok(Self::Tabloid),
            "Ledger" => Ok(Self::Ledger),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ScreenshotCaptureRequestPdfOptionsFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A0 => write!(f, "A0"),
            Self::A1 => write!(f, "A1"),
            Self::A2 => write!(f, "A2"),
            Self::A3 => write!(f, "A3"),
            Self::A4 => write!(f, "A4"),
            Self::A5 => write!(f, "A5"),
            Self::A6 => write!(f, "A6"),
            Self::Letter => write!(f, "Letter"),
            Self::Legal => write!(f, "Legal"),
            Self::Tabloid => write!(f, "Tabloid"),
            Self::Ledger => write!(f, "Ledger"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
