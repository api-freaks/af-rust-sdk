pub use crate::prelude::*;

/// Classification of the phone line.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhoneValidateResponseLineType {
    Mobile,
    FixedLine,
    FixedLineOrMobile,
    Voip,
    TollFree,
    PremiumRate,
    SharedCost,
    PersonalNumber,
    Pager,
    Uan,
    Voicemail,
    Unknown,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PhoneValidateResponseLineType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Mobile => serializer.serialize_str("MOBILE"),
            Self::FixedLine => serializer.serialize_str("FIXED_LINE"),
            Self::FixedLineOrMobile => serializer.serialize_str("FIXED_LINE_OR_MOBILE"),
            Self::Voip => serializer.serialize_str("VOIP"),
            Self::TollFree => serializer.serialize_str("TOLL_FREE"),
            Self::PremiumRate => serializer.serialize_str("PREMIUM_RATE"),
            Self::SharedCost => serializer.serialize_str("SHARED_COST"),
            Self::PersonalNumber => serializer.serialize_str("PERSONAL_NUMBER"),
            Self::Pager => serializer.serialize_str("PAGER"),
            Self::Uan => serializer.serialize_str("UAN"),
            Self::Voicemail => serializer.serialize_str("VOICEMAIL"),
            Self::Unknown => serializer.serialize_str("UNKNOWN"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PhoneValidateResponseLineType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "MOBILE" => Ok(Self::Mobile),
            "FIXED_LINE" => Ok(Self::FixedLine),
            "FIXED_LINE_OR_MOBILE" => Ok(Self::FixedLineOrMobile),
            "VOIP" => Ok(Self::Voip),
            "TOLL_FREE" => Ok(Self::TollFree),
            "PREMIUM_RATE" => Ok(Self::PremiumRate),
            "SHARED_COST" => Ok(Self::SharedCost),
            "PERSONAL_NUMBER" => Ok(Self::PersonalNumber),
            "PAGER" => Ok(Self::Pager),
            "UAN" => Ok(Self::Uan),
            "VOICEMAIL" => Ok(Self::Voicemail),
            "UNKNOWN" => Ok(Self::Unknown),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PhoneValidateResponseLineType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mobile => write!(f, "MOBILE"),
            Self::FixedLine => write!(f, "FIXED_LINE"),
            Self::FixedLineOrMobile => write!(f, "FIXED_LINE_OR_MOBILE"),
            Self::Voip => write!(f, "VOIP"),
            Self::TollFree => write!(f, "TOLL_FREE"),
            Self::PremiumRate => write!(f, "PREMIUM_RATE"),
            Self::SharedCost => write!(f, "SHARED_COST"),
            Self::PersonalNumber => write!(f, "PERSONAL_NUMBER"),
            Self::Pager => write!(f, "PAGER"),
            Self::Uan => write!(f, "UAN"),
            Self::Voicemail => write!(f, "VOICEMAIL"),
            Self::Unknown => write!(f, "UNKNOWN"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
