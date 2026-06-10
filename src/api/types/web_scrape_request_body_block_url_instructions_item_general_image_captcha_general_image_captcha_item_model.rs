pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptchaGeneralImageCaptchaItemModel
{
    MiniOcrV1,
    Model20,
    Model05,
    Model141,
    Model150,
    Model102,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize
    for WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptchaGeneralImageCaptchaItemModel
{
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::MiniOcrV1 => serializer.serialize_str("mini-ocr-v1"),
            Self::Model20 => serializer.serialize_str("Model20"),
            Self::Model05 => serializer.serialize_str("Model05"),
            Self::Model141 => serializer.serialize_str("Model141"),
            Self::Model150 => serializer.serialize_str("Model150"),
            Self::Model102 => serializer.serialize_str("Model102"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de>
    for WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptchaGeneralImageCaptchaItemModel
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "mini-ocr-v1" => Ok(Self::MiniOcrV1),
            "Model20" => Ok(Self::Model20),
            "Model05" => Ok(Self::Model05),
            "Model141" => Ok(Self::Model141),
            "Model150" => Ok(Self::Model150),
            "Model102" => Ok(Self::Model102),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display
    for WebScrapeRequestBodyBlockUrlInstructionsItemGeneralImageCaptchaGeneralImageCaptchaItemModel
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MiniOcrV1 => write!(f, "mini-ocr-v1"),
            Self::Model20 => write!(f, "Model20"),
            Self::Model05 => write!(f, "Model05"),
            Self::Model141 => write!(f, "Model141"),
            Self::Model150 => write!(f, "Model150"),
            Self::Model102 => write!(f, "Model102"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
