pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CurrentWeatherResponse {
    /// Location information
    pub location: CurrentWeatherResponseLocation,
    /// Current weather data
    #[serde(default)]
    pub current: CurrentWeatherResponseCurrent,
}

impl CurrentWeatherResponse {
    pub fn builder() -> CurrentWeatherResponseBuilder {
        <CurrentWeatherResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrentWeatherResponseBuilder {
    location: Option<CurrentWeatherResponseLocation>,
    current: Option<CurrentWeatherResponseCurrent>,
}

impl CurrentWeatherResponseBuilder {
    pub fn location(mut self, value: CurrentWeatherResponseLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn current(mut self, value: CurrentWeatherResponseCurrent) -> Self {
        self.current = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CurrentWeatherResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`location`](CurrentWeatherResponseBuilder::location)
    /// - [`current`](CurrentWeatherResponseBuilder::current)
    pub fn build(self) -> Result<CurrentWeatherResponse, BuildError> {
        Ok(CurrentWeatherResponse {
            location: self
                .location
                .ok_or_else(|| BuildError::missing_field("location"))?,
            current: self
                .current
                .ok_or_else(|| BuildError::missing_field("current"))?,
        })
    }
}
