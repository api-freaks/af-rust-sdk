pub use crate::prelude::*;

/// Difficulty level for the sentence based on the target audience
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReadabilityScoreResponseSentenceReadabilityItemReadabilityLevel {
    VeryEasy,
    Easy,
    Medium,
    Hard,
    VeryHard,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReadabilityScoreResponseSentenceReadabilityItemReadabilityLevel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::VeryEasy => serializer.serialize_str("Very Easy"),
            Self::Easy => serializer.serialize_str("Easy"),
            Self::Medium => serializer.serialize_str("Medium"),
            Self::Hard => serializer.serialize_str("Hard"),
            Self::VeryHard => serializer.serialize_str("Very Hard"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReadabilityScoreResponseSentenceReadabilityItemReadabilityLevel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Very Easy" => Ok(Self::VeryEasy),
            "Easy" => Ok(Self::Easy),
            "Medium" => Ok(Self::Medium),
            "Hard" => Ok(Self::Hard),
            "Very Hard" => Ok(Self::VeryHard),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReadabilityScoreResponseSentenceReadabilityItemReadabilityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VeryEasy => write!(f, "Very Easy"),
            Self::Easy => write!(f, "Easy"),
            Self::Medium => write!(f, "Medium"),
            Self::Hard => write!(f, "Hard"),
            Self::VeryHard => write!(f, "Very Hard"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
