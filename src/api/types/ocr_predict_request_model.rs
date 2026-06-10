pub use crate::prelude::*;

/// OCR model to use. `mini-ocr-v1` for CAPTCHA OCR, `ocr-v1` for general OCR
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OcrPredictRequestModel {
    MiniOcrV1,
    OcrV1,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OcrPredictRequestModel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::MiniOcrV1 => serializer.serialize_str("mini-ocr-v1"),
            Self::OcrV1 => serializer.serialize_str("ocr-v1"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OcrPredictRequestModel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "mini-ocr-v1" => Ok(Self::MiniOcrV1),
            "ocr-v1" => Ok(Self::OcrV1),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OcrPredictRequestModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MiniOcrV1 => write!(f, "mini-ocr-v1"),
            Self::OcrV1 => write!(f, "ocr-v1"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
