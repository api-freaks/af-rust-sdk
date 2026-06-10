pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FloodForecastResponse {
    /// Location information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<FloodForecastResponseLocation>,
    /// Flood forecast data object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast: Option<HashMap<String, FloodForecastResponseForecastValue>>,
}

impl FloodForecastResponse {
    pub fn builder() -> FloodForecastResponseBuilder {
        <FloodForecastResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FloodForecastResponseBuilder {
    location: Option<FloodForecastResponseLocation>,
    forecast: Option<HashMap<String, FloodForecastResponseForecastValue>>,
}

impl FloodForecastResponseBuilder {
    pub fn location(mut self, value: FloodForecastResponseLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn forecast(mut self, value: HashMap<String, FloodForecastResponseForecastValue>) -> Self {
        self.forecast = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FloodForecastResponse`].
    pub fn build(self) -> Result<FloodForecastResponse, BuildError> {
        Ok(FloodForecastResponse {
            location: self.location,
            forecast: self.forecast,
        })
    }
}
