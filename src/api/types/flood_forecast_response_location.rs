pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum FloodForecastResponseLocation {
    FloodForecastResponseLocationZero(FloodForecastResponseLocationZero),

    FloodForecastResponseLocationContinentCode(FloodForecastResponseLocationContinentCode),

    FloodForecastResponseLocationCity(FloodForecastResponseLocationCity),
}

impl FloodForecastResponseLocation {
    pub fn is_flood_forecast_response_location_zero(&self) -> bool {
        matches!(self, Self::FloodForecastResponseLocationZero(_))
    }

    pub fn is_flood_forecast_response_location_continent_code(&self) -> bool {
        matches!(self, Self::FloodForecastResponseLocationContinentCode(_))
    }

    pub fn is_flood_forecast_response_location_city(&self) -> bool {
        matches!(self, Self::FloodForecastResponseLocationCity(_))
    }

    pub fn as_flood_forecast_response_location_zero(
        &self,
    ) -> Option<&FloodForecastResponseLocationZero> {
        match self {
            Self::FloodForecastResponseLocationZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_flood_forecast_response_location_zero(
        self,
    ) -> Option<FloodForecastResponseLocationZero> {
        match self {
            Self::FloodForecastResponseLocationZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_flood_forecast_response_location_continent_code(
        &self,
    ) -> Option<&FloodForecastResponseLocationContinentCode> {
        match self {
            Self::FloodForecastResponseLocationContinentCode(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_flood_forecast_response_location_continent_code(
        self,
    ) -> Option<FloodForecastResponseLocationContinentCode> {
        match self {
            Self::FloodForecastResponseLocationContinentCode(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_flood_forecast_response_location_city(
        &self,
    ) -> Option<&FloodForecastResponseLocationCity> {
        match self {
            Self::FloodForecastResponseLocationCity(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_flood_forecast_response_location_city(
        self,
    ) -> Option<FloodForecastResponseLocationCity> {
        match self {
            Self::FloodForecastResponseLocationCity(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for FloodForecastResponseLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FloodForecastResponseLocationZero(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::FloodForecastResponseLocationContinentCode(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::FloodForecastResponseLocationCity(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
