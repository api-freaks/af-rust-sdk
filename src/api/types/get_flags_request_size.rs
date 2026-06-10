pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetFlagsRequestSize {
    SixteenPx,
    TwentyFourPx,
    ThirtyTwoPx,
    FortyEightPx,
    SixtyFourPx,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetFlagsRequestSize {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SixteenPx => serializer.serialize_str("16px"),
            Self::TwentyFourPx => serializer.serialize_str("24px"),
            Self::ThirtyTwoPx => serializer.serialize_str("32px"),
            Self::FortyEightPx => serializer.serialize_str("48px"),
            Self::SixtyFourPx => serializer.serialize_str("64px"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetFlagsRequestSize {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "16px" => Ok(Self::SixteenPx),
            "24px" => Ok(Self::TwentyFourPx),
            "32px" => Ok(Self::ThirtyTwoPx),
            "48px" => Ok(Self::FortyEightPx),
            "64px" => Ok(Self::SixtyFourPx),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetFlagsRequestSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SixteenPx => write!(f, "16px"),
            Self::TwentyFourPx => write!(f, "24px"),
            Self::ThirtyTwoPx => write!(f, "32px"),
            Self::FortyEightPx => write!(f, "48px"),
            Self::SixtyFourPx => write!(f, "64px"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
