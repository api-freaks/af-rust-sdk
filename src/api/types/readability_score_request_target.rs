pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReadabilityScoreRequestTarget {
    General,
    Professional,
    Academic,
    Technical,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReadabilityScoreRequestTarget {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::General => serializer.serialize_str("general"),
            Self::Professional => serializer.serialize_str("professional"),
            Self::Academic => serializer.serialize_str("academic"),
            Self::Technical => serializer.serialize_str("technical"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReadabilityScoreRequestTarget {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "general" => Ok(Self::General),
            "professional" => Ok(Self::Professional),
            "academic" => Ok(Self::Academic),
            "technical" => Ok(Self::Technical),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReadabilityScoreRequestTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::General => write!(f, "general"),
            Self::Professional => write!(f, "professional"),
            Self::Academic => write!(f, "academic"),
            Self::Technical => write!(f, "technical"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
