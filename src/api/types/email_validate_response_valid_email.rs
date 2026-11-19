pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EmailValidateResponseValidEmail {
    Valid,
    AcceptAll,
    Invalid,
    InvalidEmail,
    Unknown,
    Risky,
    AppServerBlocked,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for EmailValidateResponseValidEmail {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Valid => serializer.serialize_str("valid"),
            Self::AcceptAll => serializer.serialize_str("accept_all"),
            Self::Invalid => serializer.serialize_str("invalid"),
            Self::InvalidEmail => serializer.serialize_str("INVALID_EMAIL"),
            Self::Unknown => serializer.serialize_str("unknown"),
            Self::Risky => serializer.serialize_str("risky"),
            Self::AppServerBlocked => serializer.serialize_str("app_server_blocked"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for EmailValidateResponseValidEmail {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "valid" => Ok(Self::Valid),
            "accept_all" => Ok(Self::AcceptAll),
            "invalid" => Ok(Self::Invalid),
            "INVALID_EMAIL" => Ok(Self::InvalidEmail),
            "unknown" => Ok(Self::Unknown),
            "risky" => Ok(Self::Risky),
            "app_server_blocked" => Ok(Self::AppServerBlocked),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for EmailValidateResponseValidEmail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Valid => write!(f, "valid"),
            Self::AcceptAll => write!(f, "accept_all"),
            Self::Invalid => write!(f, "invalid"),
            Self::InvalidEmail => write!(f, "INVALID_EMAIL"),
            Self::Unknown => write!(f, "unknown"),
            Self::Risky => write!(f, "risky"),
            Self::AppServerBlocked => write!(f, "app_server_blocked"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
