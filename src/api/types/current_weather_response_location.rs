pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CurrentWeatherResponseLocation {
    CurrentWeatherResponseLocationZero(CurrentWeatherResponseLocationZero),

    CurrentWeatherResponseLocationContinentCode(CurrentWeatherResponseLocationContinentCode),

    CurrentWeatherResponseLocationCity(CurrentWeatherResponseLocationCity),
}

impl CurrentWeatherResponseLocation {
    pub fn is_current_weather_response_location_zero(&self) -> bool {
        matches!(self, Self::CurrentWeatherResponseLocationZero(_))
    }

    pub fn is_current_weather_response_location_continent_code(&self) -> bool {
        matches!(self, Self::CurrentWeatherResponseLocationContinentCode(_))
    }

    pub fn is_current_weather_response_location_city(&self) -> bool {
        matches!(self, Self::CurrentWeatherResponseLocationCity(_))
    }

    pub fn as_current_weather_response_location_zero(
        &self,
    ) -> Option<&CurrentWeatherResponseLocationZero> {
        match self {
            Self::CurrentWeatherResponseLocationZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_current_weather_response_location_zero(
        self,
    ) -> Option<CurrentWeatherResponseLocationZero> {
        match self {
            Self::CurrentWeatherResponseLocationZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_current_weather_response_location_continent_code(
        &self,
    ) -> Option<&CurrentWeatherResponseLocationContinentCode> {
        match self {
            Self::CurrentWeatherResponseLocationContinentCode(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_current_weather_response_location_continent_code(
        self,
    ) -> Option<CurrentWeatherResponseLocationContinentCode> {
        match self {
            Self::CurrentWeatherResponseLocationContinentCode(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_current_weather_response_location_city(
        &self,
    ) -> Option<&CurrentWeatherResponseLocationCity> {
        match self {
            Self::CurrentWeatherResponseLocationCity(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_current_weather_response_location_city(
        self,
    ) -> Option<CurrentWeatherResponseLocationCity> {
        match self {
            Self::CurrentWeatherResponseLocationCity(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CurrentWeatherResponseLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CurrentWeatherResponseLocationZero(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CurrentWeatherResponseLocationContinentCode(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CurrentWeatherResponseLocationCity(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
