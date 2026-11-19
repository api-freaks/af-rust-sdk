pub use crate::prelude::*;

/// Status of the subscription credits. Null if the account has no subscription.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetCreditsUsageResponseSubStatus {
    Active,
    Overdue,
    Cancelled,
    Deactivated,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetCreditsUsageResponseSubStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Active => serializer.serialize_str("active"),
            Self::Overdue => serializer.serialize_str("overdue"),
            Self::Cancelled => serializer.serialize_str("cancelled"),
            Self::Deactivated => serializer.serialize_str("deactivated"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetCreditsUsageResponseSubStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "active" => Ok(Self::Active),
            "overdue" => Ok(Self::Overdue),
            "cancelled" => Ok(Self::Cancelled),
            "deactivated" => Ok(Self::Deactivated),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetCreditsUsageResponseSubStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Overdue => write!(f, "overdue"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Deactivated => write!(f, "deactivated"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
