pub use crate::prelude::*;

/// Query parameters for weather_forecast
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WeatherForecastQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Response format returned by the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<WeatherForecastRequestFormat>,
    /// Start date for the forecast in YYYY-MM-DD format. Forecast dates must be current or future dates only. Past dates are not allowed for forecast data. The difference between startDate and endDate must not exceed 16 days.
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<NaiveDate>,
    /// End date for the forecast in YYYY-MM-DD format. Forecast dates must be current or future dates only. Past dates are not allowed for forecast data. The difference between startDate and endDate must not exceed 16 days.
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<NaiveDate>,
    /// Number of days for the forecast, from 1 to 16. Default is 7. Maximum value is 16.
    #[serde(rename = "forecastDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_days: Option<i64>,
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
    /// Precision of the forecast data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<WeatherForecastRequestPrecision>,
    /// Timezone for the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl WeatherForecastQueryRequest {
    pub fn builder() -> WeatherForecastQueryRequestBuilder {
        <WeatherForecastQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WeatherForecastQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<WeatherForecastRequestFormat>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    forecast_days: Option<i64>,
    location: Option<String>,
    lat: Option<f64>,
    long: Option<f64>,
    ip: Option<String>,
    precision: Option<WeatherForecastRequestPrecision>,
    timezone: Option<String>,
}

impl WeatherForecastQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: WeatherForecastRequestFormat) -> Self {
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

    pub fn forecast_days(mut self, value: i64) -> Self {
        self.forecast_days = Some(value);
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

    pub fn precision(mut self, value: WeatherForecastRequestPrecision) -> Self {
        self.precision = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WeatherForecastQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](WeatherForecastQueryRequestBuilder::api_key)
    pub fn build(self) -> Result<WeatherForecastQueryRequest, BuildError> {
        Ok(WeatherForecastQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            start_date: self.start_date,
            end_date: self.end_date,
            forecast_days: self.forecast_days,
            location: self.location,
            lat: self.lat,
            long: self.long,
            ip: self.ip,
            precision: self.precision,
            timezone: self.timezone,
        })
    }
}
