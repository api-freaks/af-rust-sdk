pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum BulkCurrentWeatherResponseBulkItemLocation {
    BulkCurrentWeatherResponseBulkItemLocationZero(BulkCurrentWeatherResponseBulkItemLocationZero),

    BulkCurrentWeatherResponseBulkItemLocationContinentCode(
        BulkCurrentWeatherResponseBulkItemLocationContinentCode,
    ),

    BulkCurrentWeatherResponseBulkItemLocationCity(BulkCurrentWeatherResponseBulkItemLocationCity),
}

impl BulkCurrentWeatherResponseBulkItemLocation {
    pub fn is_bulk_current_weather_response_bulk_item_location_zero(&self) -> bool {
        matches!(
            self,
            Self::BulkCurrentWeatherResponseBulkItemLocationZero(_)
        )
    }

    pub fn is_bulk_current_weather_response_bulk_item_location_continent_code(&self) -> bool {
        matches!(
            self,
            Self::BulkCurrentWeatherResponseBulkItemLocationContinentCode(_)
        )
    }

    pub fn is_bulk_current_weather_response_bulk_item_location_city(&self) -> bool {
        matches!(
            self,
            Self::BulkCurrentWeatherResponseBulkItemLocationCity(_)
        )
    }

    pub fn as_bulk_current_weather_response_bulk_item_location_zero(
        &self,
    ) -> Option<&BulkCurrentWeatherResponseBulkItemLocationZero> {
        match self {
            Self::BulkCurrentWeatherResponseBulkItemLocationZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_bulk_current_weather_response_bulk_item_location_zero(
        self,
    ) -> Option<BulkCurrentWeatherResponseBulkItemLocationZero> {
        match self {
            Self::BulkCurrentWeatherResponseBulkItemLocationZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_bulk_current_weather_response_bulk_item_location_continent_code(
        &self,
    ) -> Option<&BulkCurrentWeatherResponseBulkItemLocationContinentCode> {
        match self {
            Self::BulkCurrentWeatherResponseBulkItemLocationContinentCode(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_bulk_current_weather_response_bulk_item_location_continent_code(
        self,
    ) -> Option<BulkCurrentWeatherResponseBulkItemLocationContinentCode> {
        match self {
            Self::BulkCurrentWeatherResponseBulkItemLocationContinentCode(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_bulk_current_weather_response_bulk_item_location_city(
        &self,
    ) -> Option<&BulkCurrentWeatherResponseBulkItemLocationCity> {
        match self {
            Self::BulkCurrentWeatherResponseBulkItemLocationCity(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_bulk_current_weather_response_bulk_item_location_city(
        self,
    ) -> Option<BulkCurrentWeatherResponseBulkItemLocationCity> {
        match self {
            Self::BulkCurrentWeatherResponseBulkItemLocationCity(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for BulkCurrentWeatherResponseBulkItemLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BulkCurrentWeatherResponseBulkItemLocationZero(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::BulkCurrentWeatherResponseBulkItemLocationContinentCode(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::BulkCurrentWeatherResponseBulkItemLocationCity(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
