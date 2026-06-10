pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct HistoricalWeatherResponse {
    /// Location information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<HistoricalWeatherResponseLocation>,
    /// Historical data object of the provided date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical: Option<HistoricalWeatherResponseHistorical>,
}

impl HistoricalWeatherResponse {
    pub fn builder() -> HistoricalWeatherResponseBuilder {
        <HistoricalWeatherResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HistoricalWeatherResponseBuilder {
    location: Option<HistoricalWeatherResponseLocation>,
    historical: Option<HistoricalWeatherResponseHistorical>,
}

impl HistoricalWeatherResponseBuilder {
    pub fn location(mut self, value: HistoricalWeatherResponseLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn historical(mut self, value: HistoricalWeatherResponseHistorical) -> Self {
        self.historical = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`HistoricalWeatherResponse`].
    pub fn build(self) -> Result<HistoricalWeatherResponse, BuildError> {
        Ok(HistoricalWeatherResponse {
            location: self.location,
            historical: self.historical,
        })
    }
}
