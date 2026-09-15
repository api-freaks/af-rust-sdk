pub use crate::prelude::*;

/// Query parameters for astronomy_lookup_v2
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AstronomyLookupV2QueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response. Can be "json" or "xml".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<AstronomyLookupV2RequestFormat>,
    /// Extract astronomy information using location (preferably city)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Latitude to extract astronomy information using location coordinates
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub lat: Option<f64>,
    /// Longitude to extract astronomy information using location coordinates
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub long: Option<f64>,
    /// IPv4 or IPv6 address to extract astronomy information using IP address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Response language of "location" field in case of lookup through IP address only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<AstronomyLookupV2RequestLang>,
    /// Specific date (format YYYY-MM-DD) for which astronomy data is required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
    /// Elevation above sea level at the location, in meters. The value should be between 0 meter and a maximum value of 10,000 meters. Negative value is set to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub elevation: Option<f64>,
    /// Time zone to receive all time-based data in your preferred local time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

impl AstronomyLookupV2QueryRequest {
    pub fn builder() -> AstronomyLookupV2QueryRequestBuilder {
        <AstronomyLookupV2QueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstronomyLookupV2QueryRequestBuilder {
    api_key: Option<String>,
    format: Option<AstronomyLookupV2RequestFormat>,
    location: Option<String>,
    lat: Option<f64>,
    long: Option<f64>,
    ip: Option<String>,
    lang: Option<AstronomyLookupV2RequestLang>,
    date: Option<NaiveDate>,
    elevation: Option<f64>,
    time_zone: Option<String>,
}

impl AstronomyLookupV2QueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: AstronomyLookupV2RequestFormat) -> Self {
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

    pub fn lang(mut self, value: AstronomyLookupV2RequestLang) -> Self {
        self.lang = Some(value);
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn elevation(mut self, value: f64) -> Self {
        self.elevation = Some(value);
        self
    }

    pub fn time_zone(mut self, value: impl Into<String>) -> Self {
        self.time_zone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AstronomyLookupV2QueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](AstronomyLookupV2QueryRequestBuilder::api_key)
    pub fn build(self) -> Result<AstronomyLookupV2QueryRequest, BuildError> {
        Ok(AstronomyLookupV2QueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            location: self.location,
            lat: self.lat,
            long: self.long,
            ip: self.ip,
            lang: self.lang,
            date: self.date,
            elevation: self.elevation,
            time_zone: self.time_zone,
        })
    }
}
