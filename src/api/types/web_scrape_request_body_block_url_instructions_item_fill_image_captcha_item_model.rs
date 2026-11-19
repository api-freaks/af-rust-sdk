pub use crate::prelude::*;

/// Model used for CAPTCHA solving.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptchaItemModel {
    Model1,
    Model2,
    Model3,
    Model4,
    Model5,
    Model6,
    BasicTnImageProcessing,
    BasicPhImageProcessing,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptchaItemModel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Model1 => serializer.serialize_str("Model_1"),
            Self::Model2 => serializer.serialize_str("Model_2"),
            Self::Model3 => serializer.serialize_str("Model_3"),
            Self::Model4 => serializer.serialize_str("Model_4"),
            Self::Model5 => serializer.serialize_str("Model_5"),
            Self::Model6 => serializer.serialize_str("Model_6"),
            Self::BasicTnImageProcessing => serializer.serialize_str("basicTnImageProcessing"),
            Self::BasicPhImageProcessing => serializer.serialize_str("basicPhImageProcessing"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de>
    for WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptchaItemModel
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Model_1" => Ok(Self::Model1),
            "Model_2" => Ok(Self::Model2),
            "Model_3" => Ok(Self::Model3),
            "Model_4" => Ok(Self::Model4),
            "Model_5" => Ok(Self::Model5),
            "Model_6" => Ok(Self::Model6),
            "basicTnImageProcessing" => Ok(Self::BasicTnImageProcessing),
            "basicPhImageProcessing" => Ok(Self::BasicPhImageProcessing),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WebScrapeRequestBodyBlockUrlInstructionsItemFillImageCaptchaItemModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Model1 => write!(f, "Model_1"),
            Self::Model2 => write!(f, "Model_2"),
            Self::Model3 => write!(f, "Model_3"),
            Self::Model4 => write!(f, "Model_4"),
            Self::Model5 => write!(f, "Model_5"),
            Self::Model6 => write!(f, "Model_6"),
            Self::BasicTnImageProcessing => write!(f, "basicTnImageProcessing"),
            Self::BasicPhImageProcessing => write!(f, "basicPhImageProcessing"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
