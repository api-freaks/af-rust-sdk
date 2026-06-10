pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum HistoricalWeatherResponseLocation {
    HistoricalWeatherResponseLocationZero(HistoricalWeatherResponseLocationZero),

    HistoricalWeatherResponseLocationContinentCode(HistoricalWeatherResponseLocationContinentCode),

    HistoricalWeatherResponseLocationCity(HistoricalWeatherResponseLocationCity),
}

impl HistoricalWeatherResponseLocation {
    pub fn is_historical_weather_response_location_zero(&self) -> bool {
        matches!(self, Self::HistoricalWeatherResponseLocationZero(_))
    }

    pub fn is_historical_weather_response_location_continent_code(&self) -> bool {
        matches!(
            self,
            Self::HistoricalWeatherResponseLocationContinentCode(_)
        )
    }

    pub fn is_historical_weather_response_location_city(&self) -> bool {
        matches!(self, Self::HistoricalWeatherResponseLocationCity(_))
    }

    pub fn as_historical_weather_response_location_zero(
        &self,
    ) -> Option<&HistoricalWeatherResponseLocationZero> {
        match self {
            Self::HistoricalWeatherResponseLocationZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_historical_weather_response_location_zero(
        self,
    ) -> Option<HistoricalWeatherResponseLocationZero> {
        match self {
            Self::HistoricalWeatherResponseLocationZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_historical_weather_response_location_continent_code(
        &self,
    ) -> Option<&HistoricalWeatherResponseLocationContinentCode> {
        match self {
            Self::HistoricalWeatherResponseLocationContinentCode(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_historical_weather_response_location_continent_code(
        self,
    ) -> Option<HistoricalWeatherResponseLocationContinentCode> {
        match self {
            Self::HistoricalWeatherResponseLocationContinentCode(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_historical_weather_response_location_city(
        &self,
    ) -> Option<&HistoricalWeatherResponseLocationCity> {
        match self {
            Self::HistoricalWeatherResponseLocationCity(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_historical_weather_response_location_city(
        self,
    ) -> Option<HistoricalWeatherResponseLocationCity> {
        match self {
            Self::HistoricalWeatherResponseLocationCity(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for HistoricalWeatherResponseLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HistoricalWeatherResponseLocationZero(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::HistoricalWeatherResponseLocationContinentCode(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::HistoricalWeatherResponseLocationCity(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
