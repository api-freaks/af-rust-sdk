pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum WeatherTimeSeriesResponseLocation {
    WeatherTimeSeriesResponseLocationZero(WeatherTimeSeriesResponseLocationZero),

    WeatherTimeSeriesResponseLocationContinentCode(WeatherTimeSeriesResponseLocationContinentCode),

    WeatherTimeSeriesResponseLocationCity(WeatherTimeSeriesResponseLocationCity),
}

impl WeatherTimeSeriesResponseLocation {
    pub fn is_weather_time_series_response_location_zero(&self) -> bool {
        matches!(self, Self::WeatherTimeSeriesResponseLocationZero(_))
    }

    pub fn is_weather_time_series_response_location_continent_code(&self) -> bool {
        matches!(
            self,
            Self::WeatherTimeSeriesResponseLocationContinentCode(_)
        )
    }

    pub fn is_weather_time_series_response_location_city(&self) -> bool {
        matches!(self, Self::WeatherTimeSeriesResponseLocationCity(_))
    }

    pub fn as_weather_time_series_response_location_zero(
        &self,
    ) -> Option<&WeatherTimeSeriesResponseLocationZero> {
        match self {
            Self::WeatherTimeSeriesResponseLocationZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_weather_time_series_response_location_zero(
        self,
    ) -> Option<WeatherTimeSeriesResponseLocationZero> {
        match self {
            Self::WeatherTimeSeriesResponseLocationZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_weather_time_series_response_location_continent_code(
        &self,
    ) -> Option<&WeatherTimeSeriesResponseLocationContinentCode> {
        match self {
            Self::WeatherTimeSeriesResponseLocationContinentCode(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_weather_time_series_response_location_continent_code(
        self,
    ) -> Option<WeatherTimeSeriesResponseLocationContinentCode> {
        match self {
            Self::WeatherTimeSeriesResponseLocationContinentCode(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_weather_time_series_response_location_city(
        &self,
    ) -> Option<&WeatherTimeSeriesResponseLocationCity> {
        match self {
            Self::WeatherTimeSeriesResponseLocationCity(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_weather_time_series_response_location_city(
        self,
    ) -> Option<WeatherTimeSeriesResponseLocationCity> {
        match self {
            Self::WeatherTimeSeriesResponseLocationCity(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for WeatherTimeSeriesResponseLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WeatherTimeSeriesResponseLocationZero(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WeatherTimeSeriesResponseLocationContinentCode(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::WeatherTimeSeriesResponseLocationCity(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
