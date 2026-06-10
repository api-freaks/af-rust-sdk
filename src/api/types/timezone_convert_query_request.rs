pub use crate::prelude::*;

/// Query parameters for timezone_convert
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimezoneConvertQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response .
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<TimezoneConvertRequestFormat>,
    /// Time to convert in `yyyy-MM-dd HH:mm` or `yyyy-MM-dd HH:mm:ss` format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// Source timezone name (e.g., `Asia/Kolkata`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tz_from: Option<String>,
    /// Target timezone name (e.g., `America/New_York`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tz_to: Option<String>,
    /// Latitude of source location.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub lat_from: Option<f64>,
    /// Longitude of source location.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub long_from: Option<f64>,
    /// Latitude of target location.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub lat_to: Option<f64>,
    /// Longitude of target location.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub long_to: Option<f64>,
    /// From location (city/country).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_from: Option<String>,
    /// To location (city/country).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_to: Option<String>,
    /// From IATA airport code (e.g., JFK).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iata_from: Option<String>,
    /// To IATA airport code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iata_to: Option<String>,
    /// From ICAO airport code (e.g., KJFK).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icao_from: Option<String>,
    /// To ICAO airport code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icao_to: Option<String>,
    /// From UN/LO CODE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locode_from: Option<String>,
    /// To UN/LO CODE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locode_to: Option<String>,
}

impl TimezoneConvertQueryRequest {
    pub fn builder() -> TimezoneConvertQueryRequestBuilder {
        <TimezoneConvertQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimezoneConvertQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<TimezoneConvertRequestFormat>,
    time: Option<String>,
    tz_from: Option<String>,
    tz_to: Option<String>,
    lat_from: Option<f64>,
    long_from: Option<f64>,
    lat_to: Option<f64>,
    long_to: Option<f64>,
    location_from: Option<String>,
    location_to: Option<String>,
    iata_from: Option<String>,
    iata_to: Option<String>,
    icao_from: Option<String>,
    icao_to: Option<String>,
    locode_from: Option<String>,
    locode_to: Option<String>,
}

impl TimezoneConvertQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: TimezoneConvertRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn time(mut self, value: impl Into<String>) -> Self {
        self.time = Some(value.into());
        self
    }

    pub fn tz_from(mut self, value: impl Into<String>) -> Self {
        self.tz_from = Some(value.into());
        self
    }

    pub fn tz_to(mut self, value: impl Into<String>) -> Self {
        self.tz_to = Some(value.into());
        self
    }

    pub fn lat_from(mut self, value: f64) -> Self {
        self.lat_from = Some(value);
        self
    }

    pub fn long_from(mut self, value: f64) -> Self {
        self.long_from = Some(value);
        self
    }

    pub fn lat_to(mut self, value: f64) -> Self {
        self.lat_to = Some(value);
        self
    }

    pub fn long_to(mut self, value: f64) -> Self {
        self.long_to = Some(value);
        self
    }

    pub fn location_from(mut self, value: impl Into<String>) -> Self {
        self.location_from = Some(value.into());
        self
    }

    pub fn location_to(mut self, value: impl Into<String>) -> Self {
        self.location_to = Some(value.into());
        self
    }

    pub fn iata_from(mut self, value: impl Into<String>) -> Self {
        self.iata_from = Some(value.into());
        self
    }

    pub fn iata_to(mut self, value: impl Into<String>) -> Self {
        self.iata_to = Some(value.into());
        self
    }

    pub fn icao_from(mut self, value: impl Into<String>) -> Self {
        self.icao_from = Some(value.into());
        self
    }

    pub fn icao_to(mut self, value: impl Into<String>) -> Self {
        self.icao_to = Some(value.into());
        self
    }

    pub fn locode_from(mut self, value: impl Into<String>) -> Self {
        self.locode_from = Some(value.into());
        self
    }

    pub fn locode_to(mut self, value: impl Into<String>) -> Self {
        self.locode_to = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimezoneConvertQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](TimezoneConvertQueryRequestBuilder::api_key)
    pub fn build(self) -> Result<TimezoneConvertQueryRequest, BuildError> {
        Ok(TimezoneConvertQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            time: self.time,
            tz_from: self.tz_from,
            tz_to: self.tz_to,
            lat_from: self.lat_from,
            long_from: self.long_from,
            lat_to: self.lat_to,
            long_to: self.long_to,
            location_from: self.location_from,
            location_to: self.location_to,
            iata_from: self.iata_from,
            iata_to: self.iata_to,
            icao_from: self.icao_from,
            icao_to: self.icao_to,
            locode_from: self.locode_from,
            locode_to: self.locode_to,
        })
    }
}
