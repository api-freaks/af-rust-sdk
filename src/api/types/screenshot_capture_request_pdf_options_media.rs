pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScreenshotCaptureRequestPdfOptionsMedia {
    Print,
    Screen,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ScreenshotCaptureRequestPdfOptionsMedia {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Print => serializer.serialize_str("print"),
            Self::Screen => serializer.serialize_str("screen"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ScreenshotCaptureRequestPdfOptionsMedia {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "print" => Ok(Self::Print),
            "screen" => Ok(Self::Screen),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ScreenshotCaptureRequestPdfOptionsMedia {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Print => write!(f, "print"),
            Self::Screen => write!(f, "screen"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
