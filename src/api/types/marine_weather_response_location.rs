pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MarineWeatherResponseLocation {
    MarineWeatherResponseLocationZero(MarineWeatherResponseLocationZero),

    MarineWeatherResponseLocationContinentCode(MarineWeatherResponseLocationContinentCode),

    MarineWeatherResponseLocationCity(MarineWeatherResponseLocationCity),
}

impl MarineWeatherResponseLocation {
    pub fn is_marine_weather_response_location_zero(&self) -> bool {
        matches!(self, Self::MarineWeatherResponseLocationZero(_))
    }

    pub fn is_marine_weather_response_location_continent_code(&self) -> bool {
        matches!(self, Self::MarineWeatherResponseLocationContinentCode(_))
    }

    pub fn is_marine_weather_response_location_city(&self) -> bool {
        matches!(self, Self::MarineWeatherResponseLocationCity(_))
    }

    pub fn as_marine_weather_response_location_zero(
        &self,
    ) -> Option<&MarineWeatherResponseLocationZero> {
        match self {
            Self::MarineWeatherResponseLocationZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_marine_weather_response_location_zero(
        self,
    ) -> Option<MarineWeatherResponseLocationZero> {
        match self {
            Self::MarineWeatherResponseLocationZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_marine_weather_response_location_continent_code(
        &self,
    ) -> Option<&MarineWeatherResponseLocationContinentCode> {
        match self {
            Self::MarineWeatherResponseLocationContinentCode(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_marine_weather_response_location_continent_code(
        self,
    ) -> Option<MarineWeatherResponseLocationContinentCode> {
        match self {
            Self::MarineWeatherResponseLocationContinentCode(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_marine_weather_response_location_city(
        &self,
    ) -> Option<&MarineWeatherResponseLocationCity> {
        match self {
            Self::MarineWeatherResponseLocationCity(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_marine_weather_response_location_city(
        self,
    ) -> Option<MarineWeatherResponseLocationCity> {
        match self {
            Self::MarineWeatherResponseLocationCity(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for MarineWeatherResponseLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MarineWeatherResponseLocationZero(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::MarineWeatherResponseLocationContinentCode(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::MarineWeatherResponseLocationCity(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
