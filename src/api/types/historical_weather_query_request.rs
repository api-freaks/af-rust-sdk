pub use crate::prelude::*;

/// Query parameters for historical_weather
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct HistoricalWeatherQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Response format returned by the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<HistoricalWeatherRequestFormat>,
    /// Specific date for which to fetch weather data in YYYY-MM-DD format. Historical dates must be past dates only. Current or future dates are not allowed for historical data. Data available from 1940 onwards.
    #[serde(default)]
    pub date: NaiveDate,
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
    /// Precision of the historical data. **Note:** 'daily' returns daily aggregates, 'hourly' returns hourly data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<HistoricalWeatherRequestPrecision>,
    /// Timezone for the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl HistoricalWeatherQueryRequest {
    pub fn builder() -> HistoricalWeatherQueryRequestBuilder {
        <HistoricalWeatherQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HistoricalWeatherQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<HistoricalWeatherRequestFormat>,
    date: Option<NaiveDate>,
    location: Option<String>,
    lat: Option<f64>,
    long: Option<f64>,
    ip: Option<String>,
    precision: Option<HistoricalWeatherRequestPrecision>,
    timezone: Option<String>,
}

impl HistoricalWeatherQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: HistoricalWeatherRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
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

    pub fn precision(mut self, value: HistoricalWeatherRequestPrecision) -> Self {
        self.precision = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`HistoricalWeatherQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](HistoricalWeatherQueryRequestBuilder::api_key)
    /// - [`date`](HistoricalWeatherQueryRequestBuilder::date)
    pub fn build(self) -> Result<HistoricalWeatherQueryRequest, BuildError> {
        Ok(HistoricalWeatherQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            location: self.location,
            lat: self.lat,
            long: self.long,
            ip: self.ip,
            precision: self.precision,
            timezone: self.timezone,
        })
    }
}
