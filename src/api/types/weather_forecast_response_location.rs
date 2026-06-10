pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum WeatherForecastResponseLocation {
    WeatherForecastResponseLocationZero(WeatherForecastResponseLocationZero),

    WeatherForecastResponseLocationContinentCode(WeatherForecastResponseLocationContinentCode),

    WeatherForecastResponseLocationCity(WeatherForecastResponseLocationCity),
}

impl WeatherForecastResponseLocation {
    pub fn is_weather_forecast_response_location_zero(&self) -> bool {
        matches!(self, Self::WeatherForecastResponseLocationZero(_))
    }

    pub fn is_weather_forecast_response_location_continent_code(&self) -> bool {
        matches!(self, Self::WeatherForecastResponseLocationContinentCode(_))
    }

    pub fn is_weather_forecast_response_location_city(&self) -> bool {
        matches!(self, Self::WeatherForecastResponseLocationCity(_))
    }

    pub fn as_weather_forecast_response_location_zero(
        &self,
    ) -> Option<&WeatherForecastResponseLocationZero> {
        match self {
            Self::WeatherForecastResponseLocationZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_weather_forecast_response_location_zero(
        self,
    ) -> Option<WeatherForecastResponseLocationZero> {
        match self {
            Self::WeatherForecastResponseLocationZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_weather_forecast_response_location_continent_code(
        &self,
    ) -> Option<&WeatherForecastResponseLocationContinentCode> {
        match self {
            Self::WeatherForecastResponseLocationContinentCode(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_weather_forecast_response_location_continent_code(
        self,
    ) -> Option<WeatherForecastResponseLocationContinentCode> {
        match self {
            Self::WeatherForecastResponseLocationContinentCode(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_weather_forecast_response_location_city(
        &self,
    ) -> Option<&WeatherForecastResponseLocationCity> {
        match self {
            Self::WeatherForecastResponseLocationCity(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_weather_forecast_response_location_city(
        self,
    ) -> Option<WeatherForecastResponseLocationCity> {
        match self {
            Self::WeatherForecastResponseLocationCity(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for WeatherForecastResponseLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WeatherForecastResponseLocationZero(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WeatherForecastResponseLocationContinentCode(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WeatherForecastResponseLocationCity(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
