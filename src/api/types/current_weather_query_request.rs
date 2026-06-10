pub use crate::prelude::*;

/// Query parameters for current_weather
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CurrentWeatherQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Response format returned by the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<CurrentWeatherRequestFormat>,
    /// City name, place name, or full address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Latitude of the location.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub lat: Option<f64>,
    /// Longitude of the location.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub long: Option<f64>,
    /// IP(v4 or v6) address for location inference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Timezone for the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl CurrentWeatherQueryRequest {
    pub fn builder() -> CurrentWeatherQueryRequestBuilder {
        <CurrentWeatherQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrentWeatherQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<CurrentWeatherRequestFormat>,
    location: Option<String>,
    lat: Option<f64>,
    long: Option<f64>,
    ip: Option<String>,
    timezone: Option<String>,
}

impl CurrentWeatherQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: CurrentWeatherRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn location(mut self, value: impl Into<String>) -> Self {
        self.location = Some(value.into());
        self
    }

    pub fn lat(mut self, value: f64) -> Self {
        self.lat = Some(value);
        self
    }

    pub fn long(mut self, value: f64) -> Self {
        self.long = Some(value);
        self
    }

    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CurrentWeatherQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](CurrentWeatherQueryRequestBuilder::api_key)
    pub fn build(self) -> Result<CurrentWeatherQueryRequest, BuildError> {
        Ok(CurrentWeatherQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            location: self.location,
            lat: self.lat,
            long: self.long,
            ip: self.ip,
            timezone: self.timezone,
        })
    }
}
