pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WeatherForecastResponse {
    /// Location information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<WeatherForecastResponseLocation>,
    /// Forecast data object of the provided date range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast: Option<HashMap<String, WeatherForecastResponseForecastValue>>,
}

impl WeatherForecastResponse {
    pub fn builder() -> WeatherForecastResponseBuilder {
        <WeatherForecastResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WeatherForecastResponseBuilder {
    location: Option<WeatherForecastResponseLocation>,
    forecast: Option<HashMap<String, WeatherForecastResponseForecastValue>>,
}

impl WeatherForecastResponseBuilder {
    pub fn location(mut self, value: WeatherForecastResponseLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn forecast(
        mut self,
        value: HashMap<String, WeatherForecastResponseForecastValue>,
    ) -> Self {
        self.forecast = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WeatherForecastResponse`].
    pub fn build(self) -> Result<WeatherForecastResponse, BuildError> {
        Ok(WeatherForecastResponse {
            location: self.location,
            forecast: self.forecast,
        })
    }
}
