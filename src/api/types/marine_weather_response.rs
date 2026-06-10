pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MarineWeatherResponse {
    /// Location information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<MarineWeatherResponseLocation>,
    /// Current marine data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<MarineWeatherResponseCurrent>,
    /// Marine forecast data object keyed by date in YYYY-MM-DD format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast: Option<HashMap<String, MarineWeatherResponseForecastValue>>,
}

impl MarineWeatherResponse {
    pub fn builder() -> MarineWeatherResponseBuilder {
        <MarineWeatherResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MarineWeatherResponseBuilder {
    location: Option<MarineWeatherResponseLocation>,
    current: Option<MarineWeatherResponseCurrent>,
    forecast: Option<HashMap<String, MarineWeatherResponseForecastValue>>,
}

impl MarineWeatherResponseBuilder {
    pub fn location(mut self, value: MarineWeatherResponseLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn current(mut self, value: MarineWeatherResponseCurrent) -> Self {
        self.current = Some(value);
        self
    }

    pub fn forecast(mut self, value: HashMap<String, MarineWeatherResponseForecastValue>) -> Self {
        self.forecast = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MarineWeatherResponse`].
    pub fn build(self) -> Result<MarineWeatherResponse, BuildError> {
        Ok(MarineWeatherResponse {
            location: self.location,
            current: self.current,
            forecast: self.forecast,
        })
    }
}
