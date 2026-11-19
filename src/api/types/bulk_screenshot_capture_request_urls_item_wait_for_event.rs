pub use crate::prelude::*;

/// Controls when rendering starts based on page load lifecycle. `load` (all resources), `domcontentloaded` (HTML parsed), `networkidle` (no network activity for 500ms).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BulkScreenshotCaptureRequestUrlsItemWaitForEvent {
    Load,
    Domcontentloaded,
    Networkidle,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BulkScreenshotCaptureRequestUrlsItemWaitForEvent {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Load => serializer.serialize_str("load"),
            Self::Domcontentloaded => serializer.serialize_str("domcontentloaded"),
            Self::Networkidle => serializer.serialize_str("networkidle"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BulkScreenshotCaptureRequestUrlsItemWaitForEvent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "load" => Ok(Self::Load),
            "domcontentloaded" => Ok(Self::Domcontentloaded),
            "networkidle" => Ok(Self::Networkidle),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BulkScreenshotCaptureRequestUrlsItemWaitForEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Load => write!(f, "load"),
            Self::Domcontentloaded => write!(f, "domcontentloaded"),
            Self::Networkidle => write!(f, "networkidle"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
