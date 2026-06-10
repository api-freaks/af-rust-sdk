pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WeatherTimeSeriesResponse {
    /// Location information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<WeatherTimeSeriesResponseLocation>,
    /// Historical data object of the provided date range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical: Option<HashMap<String, WeatherTimeSeriesResponseHistoricalValue>>,
}

impl WeatherTimeSeriesResponse {
    pub fn builder() -> WeatherTimeSeriesResponseBuilder {
        <WeatherTimeSeriesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WeatherTimeSeriesResponseBuilder {
    location: Option<WeatherTimeSeriesResponseLocation>,
    historical: Option<HashMap<String, WeatherTimeSeriesResponseHistoricalValue>>,
}

impl WeatherTimeSeriesResponseBuilder {
    pub fn location(mut self, value: WeatherTimeSeriesResponseLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn historical(
        mut self,
        value: HashMap<String, WeatherTimeSeriesResponseHistoricalValue>,
    ) -> Self {
        self.historical = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WeatherTimeSeriesResponse`].
    pub fn build(self) -> Result<WeatherTimeSeriesResponse, BuildError> {
        Ok(WeatherTimeSeriesResponse {
            location: self.location,
            historical: self.historical,
        })
    }
}
