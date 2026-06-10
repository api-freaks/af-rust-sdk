pub use crate::prelude::*;

/// Query parameters for geolocation_lookup
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeolocationLookupQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<GeolocationLookupRequestFormat>,
    /// IPv4, IPv6, or hostname for geolocation lookup
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Response language for location fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<GeolocationLookupRequestLang>,
    /// Comma separated list of fields to include in response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    /// Comma separated list of fields to exclude from response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludes: Option<String>,
    /// Additional data to include (location, network, security, currency, time_zone, user_agent, country_metadata , hostname, liveHostname, hostnameFallbackLivet)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
}

impl GeolocationLookupQueryRequest {
    pub fn builder() -> GeolocationLookupQueryRequestBuilder {
        <GeolocationLookupQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeolocationLookupQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<GeolocationLookupRequestFormat>,
    ip: Option<String>,
    lang: Option<GeolocationLookupRequestLang>,
    fields: Option<String>,
    excludes: Option<String>,
    include: Option<String>,
}

impl GeolocationLookupQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: GeolocationLookupRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn lang(mut self, value: GeolocationLookupRequestLang) -> Self {
        self.lang = Some(value);
        self
    }

    pub fn fields(mut self, value: impl Into<String>) -> Self {
        self.fields = Some(value.into());
        self
    }

    pub fn excludes(mut self, value: impl Into<String>) -> Self {
        self.excludes = Some(value.into());
        self
    }

    pub fn include(mut self, value: impl Into<String>) -> Self {
        self.include = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GeolocationLookupQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](GeolocationLookupQueryRequestBuilder::api_key)
    pub fn build(self) -> Result<GeolocationLookupQueryRequest, BuildError> {
        Ok(GeolocationLookupQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            ip: self.ip,
            lang: self.lang,
            fields: self.fields,
            excludes: self.excludes,
            include: self.include,
        })
    }
}
