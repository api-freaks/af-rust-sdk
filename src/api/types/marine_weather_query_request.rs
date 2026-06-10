pub use crate::prelude::*;

/// Query parameters for marine_weather
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MarineWeatherQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Response format returned by the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<MarineWeatherRequestFormat>,
    /// Starting date for marine forecast data in YYYY-MM-DD format. Forecast dates must be current or future dates only. Past dates are not allowed for forecast data. The difference between endDate and startDate must not exceed 16 days.
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<NaiveDate>,
    /// End date for marine forecast data in YYYY-MM-DD format. Forecast dates must be current or future dates only. Past dates are not allowed for forecast data. The difference between endDate and startDate must not exceed 16 days.
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<NaiveDate>,
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
    /// Precision of the marine data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<MarineWeatherRequestPrecision>,
    /// Timezone for the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl MarineWeatherQueryRequest {
    pub fn builder() -> MarineWeatherQueryRequestBuilder {
        <MarineWeatherQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MarineWeatherQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<MarineWeatherRequestFormat>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    location: Option<String>,
    lat: Option<f64>,
    long: Option<f64>,
    ip: Option<String>,
    precision: Option<MarineWeatherRequestPrecision>,
    timezone: Option<String>,
}

impl MarineWeatherQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: MarineWeatherRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn start_date(mut self, value: NaiveDate) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn end_date(mut self, value: NaiveDate) -> Self {
        self.end_date = Some(value);
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

    pub fn precision(mut self, value: MarineWeatherRequestPrecision) -> Self {
        self.precision = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MarineWeatherQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](MarineWeatherQueryRequestBuilder::api_key)
    pub fn build(self) -> Result<MarineWeatherQueryRequest, BuildError> {
        Ok(MarineWeatherQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            start_date: self.start_date,
            end_date: self.end_date,
            location: self.location,
            lat: self.lat,
            long: self.long,
            ip: self.ip,
            precision: self.precision,
            timezone: self.timezone,
        })
    }
}
