pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AirQualityResponse {
    /// Location information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<AirQualityResponseLocation>,
    /// Current air quality data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<AirQualityResponseCurrent>,
    /// Air quality forecast data object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast: Option<HashMap<String, AirQualityResponseForecastValue>>,
}

impl AirQualityResponse {
    pub fn builder() -> AirQualityResponseBuilder {
        <AirQualityResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AirQualityResponseBuilder {
    location: Option<AirQualityResponseLocation>,
    current: Option<AirQualityResponseCurrent>,
    forecast: Option<HashMap<String, AirQualityResponseForecastValue>>,
}

impl AirQualityResponseBuilder {
    pub fn location(mut self, value: AirQualityResponseLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn current(mut self, value: AirQualityResponseCurrent) -> Self {
        self.current = Some(value);
        self
    }

    pub fn forecast(mut self, value: HashMap<String, AirQualityResponseForecastValue>) -> Self {
        self.forecast = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AirQualityResponse`].
    pub fn build(self) -> Result<AirQualityResponse, BuildError> {
        Ok(AirQualityResponse {
            location: self.location,
            current: self.current,
            forecast: self.forecast,
        })
    }
}
