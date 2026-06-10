pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum AirQualityResponseLocation {
    AirQualityResponseLocationZero(AirQualityResponseLocationZero),

    AirQualityResponseLocationContinentCode(AirQualityResponseLocationContinentCode),

    AirQualityResponseLocationCity(AirQualityResponseLocationCity),
}

impl AirQualityResponseLocation {
    pub fn is_air_quality_response_location_zero(&self) -> bool {
        matches!(self, Self::AirQualityResponseLocationZero(_))
    }

    pub fn is_air_quality_response_location_continent_code(&self) -> bool {
        matches!(self, Self::AirQualityResponseLocationContinentCode(_))
    }

    pub fn is_air_quality_response_location_city(&self) -> bool {
        matches!(self, Self::AirQualityResponseLocationCity(_))
    }

    pub fn as_air_quality_response_location_zero(&self) -> Option<&AirQualityResponseLocationZero> {
        match self {
            Self::AirQualityResponseLocationZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_air_quality_response_location_zero(self) -> Option<AirQualityResponseLocationZero> {
        match self {
            Self::AirQualityResponseLocationZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_air_quality_response_location_continent_code(
        &self,
    ) -> Option<&AirQualityResponseLocationContinentCode> {
        match self {
            Self::AirQualityResponseLocationContinentCode(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_air_quality_response_location_continent_code(
        self,
    ) -> Option<AirQualityResponseLocationContinentCode> {
        match self {
            Self::AirQualityResponseLocationContinentCode(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_air_quality_response_location_city(&self) -> Option<&AirQualityResponseLocationCity> {
        match self {
            Self::AirQualityResponseLocationCity(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_air_quality_response_location_city(self) -> Option<AirQualityResponseLocationCity> {
        match self {
            Self::AirQualityResponseLocationCity(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for AirQualityResponseLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AirQualityResponseLocationZero(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::AirQualityResponseLocationContinentCode(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::AirQualityResponseLocationCity(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
