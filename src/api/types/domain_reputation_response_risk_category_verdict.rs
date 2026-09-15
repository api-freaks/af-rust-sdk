pub use crate::prelude::*;

/// Final verdict of the risk assessment.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DomainReputationResponseRiskCategoryVerdict {
    Safe,
    Suspicious,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DomainReputationResponseRiskCategoryVerdict {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Safe => serializer.serialize_str("safe"),
            Self::Suspicious => serializer.serialize_str("suspicious"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DomainReputationResponseRiskCategoryVerdict {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "safe" => Ok(Self::Safe),
            "suspicious" => Ok(Self::Suspicious),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DomainReputationResponseRiskCategoryVerdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Safe => write!(f, "safe"),
            Self::Suspicious => write!(f, "suspicious"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
