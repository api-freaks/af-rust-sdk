pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommodityIntradayRatesV2RequestInterval {
    OneM,
    FiveM,
    TenM,
    FifteenM,
    ThirtyM,
    FortyFiveM,
    OneH,
    TwoH,
    FourH,
    EightH,
    TwelveH,
    OneD,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CommodityIntradayRatesV2RequestInterval {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::OneM => serializer.serialize_str("1m"),
            Self::FiveM => serializer.serialize_str("5m"),
            Self::TenM => serializer.serialize_str("10m"),
            Self::FifteenM => serializer.serialize_str("15m"),
            Self::ThirtyM => serializer.serialize_str("30m"),
            Self::FortyFiveM => serializer.serialize_str("45m"),
            Self::OneH => serializer.serialize_str("1h"),
            Self::TwoH => serializer.serialize_str("2h"),
            Self::FourH => serializer.serialize_str("4h"),
            Self::EightH => serializer.serialize_str("8h"),
            Self::TwelveH => serializer.serialize_str("12h"),
            Self::OneD => serializer.serialize_str("1d"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CommodityIntradayRatesV2RequestInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "1m" => Ok(Self::OneM),
            "5m" => Ok(Self::FiveM),
            "10m" => Ok(Self::TenM),
            "15m" => Ok(Self::FifteenM),
            "30m" => Ok(Self::ThirtyM),
            "45m" => Ok(Self::FortyFiveM),
            "1h" => Ok(Self::OneH),
            "2h" => Ok(Self::TwoH),
            "4h" => Ok(Self::FourH),
            "8h" => Ok(Self::EightH),
            "12h" => Ok(Self::TwelveH),
            "1d" => Ok(Self::OneD),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CommodityIntradayRatesV2RequestInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OneM => write!(f, "1m"),
            Self::FiveM => write!(f, "5m"),
            Self::TenM => write!(f, "10m"),
            Self::FifteenM => write!(f, "15m"),
            Self::ThirtyM => write!(f, "30m"),
            Self::FortyFiveM => write!(f, "45m"),
            Self::OneH => write!(f, "1h"),
            Self::TwoH => write!(f, "2h"),
            Self::FourH => write!(f, "4h"),
            Self::EightH => write!(f, "8h"),
            Self::TwelveH => write!(f, "12h"),
            Self::OneD => write!(f, "1d"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
