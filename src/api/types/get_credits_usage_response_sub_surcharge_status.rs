pub use crate::prelude::*;

/// Status of the subscription surcharge (overage) credits. Null if the account has no subscription.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetCreditsUsageResponseSubSurchargeStatus {
    None,
    PaymentAdded,
    PaymentCleared,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetCreditsUsageResponseSubSurchargeStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::None => serializer.serialize_str("none"),
            Self::PaymentAdded => serializer.serialize_str("payment_added"),
            Self::PaymentCleared => serializer.serialize_str("payment_cleared"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetCreditsUsageResponseSubSurchargeStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "none" => Ok(Self::None),
            "payment_added" => Ok(Self::PaymentAdded),
            "payment_cleared" => Ok(Self::PaymentCleared),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetCreditsUsageResponseSubSurchargeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::PaymentAdded => write!(f, "payment_added"),
            Self::PaymentCleared => write!(f, "payment_cleared"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
