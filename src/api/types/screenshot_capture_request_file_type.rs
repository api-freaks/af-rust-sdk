pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScreenshotCaptureRequestFileType {
    Png,
    Jpeg,
    Webp,
    Pdf,
    Mp4,
    Gif,
    Webm,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ScreenshotCaptureRequestFileType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Png => serializer.serialize_str("PNG"),
            Self::Jpeg => serializer.serialize_str("JPEG"),
            Self::Webp => serializer.serialize_str("WEBP"),
            Self::Pdf => serializer.serialize_str("PDF"),
            Self::Mp4 => serializer.serialize_str("mp4"),
            Self::Gif => serializer.serialize_str("gif"),
            Self::Webm => serializer.serialize_str("webm"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ScreenshotCaptureRequestFileType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "PNG" => Ok(Self::Png),
            "JPEG" => Ok(Self::Jpeg),
            "WEBP" => Ok(Self::Webp),
            "PDF" => Ok(Self::Pdf),
            "mp4" => Ok(Self::Mp4),
            "gif" => Ok(Self::Gif),
            "webm" => Ok(Self::Webm),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ScreenshotCaptureRequestFileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Png => write!(f, "PNG"),
            Self::Jpeg => write!(f, "JPEG"),
            Self::Webp => write!(f, "WEBP"),
            Self::Pdf => write!(f, "PDF"),
            Self::Mp4 => write!(f, "mp4"),
            Self::Gif => write!(f, "gif"),
            Self::Webm => write!(f, "webm"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
