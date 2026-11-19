pub use crate::prelude::*;

/// Scrolling speed: fast, normal (default), or slow. Affects render time and output pacing.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BulkScreenshotCaptureRequestUrlsItemScrollSpeed {
    Normal,
    Fast,
    Slow,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BulkScreenshotCaptureRequestUrlsItemScrollSpeed {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Normal => serializer.serialize_str("normal"),
            Self::Fast => serializer.serialize_str("fast"),
            Self::Slow => serializer.serialize_str("slow"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BulkScreenshotCaptureRequestUrlsItemScrollSpeed {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "normal" => Ok(Self::Normal),
            "fast" => Ok(Self::Fast),
            "slow" => Ok(Self::Slow),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BulkScreenshotCaptureRequestUrlsItemScrollSpeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Fast => write!(f, "fast"),
            Self::Slow => write!(f, "slow"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
