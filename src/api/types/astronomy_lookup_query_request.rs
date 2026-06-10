pub use crate::prelude::*;

/// Query parameters for astronomy_lookup
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AstronomyLookupQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<AstronomyLookupRequestFormat>,
    /// Location name or address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Latitude for location coordinates
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub lat: Option<f64>,
    /// Longitude for location coordinates
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub long: Option<f64>,
    /// IP address for location detection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// Date for astronomy data (YYYY-MM-DD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
    /// Timezone of the location for which astronomy data is required
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub elevation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

impl AstronomyLookupQueryRequest {
    pub fn builder() -> AstronomyLookupQueryRequestBuilder {
        <AstronomyLookupQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstronomyLookupQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<AstronomyLookupRequestFormat>,
    location: Option<String>,
    lat: Option<f64>,
    long: Option<f64>,
    ip: Option<String>,
    lang: Option<String>,
    date: Option<NaiveDate>,
    elevation: Option<f64>,
    time_zone: Option<String>,
}

impl AstronomyLookupQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: AstronomyLookupRequestFormat) -> Self {
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

    pub fn lang(mut self, value: impl Into<String>) -> Self {
        self.lang = Some(value.into());
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

    /// Consumes the builder and constructs a [`AstronomyLookupQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](AstronomyLookupQueryRequestBuilder::api_key)
    pub fn build(self) -> Result<AstronomyLookupQueryRequest, BuildError> {
        Ok(AstronomyLookupQueryRequest {
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
