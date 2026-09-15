pub use crate::prelude::*;

/// Query parameters for geolocation_lookup_v2
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeolocationLookupV2QueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<GeolocationLookupV2RequestFormat>,
    /// IPv4, IPv6, or hostname for geolocation lookup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Response language for location fields. Default: en.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<GeolocationLookupV2RequestLang>,
    /// Comma-separated list of fields to include in response. For example, `location` includes all location fields, `location.city` is a specific field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    /// Comma-separated list of fields to exclude from response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludes: Option<String>,
    /// Comma-separated list of additional data modules to include. Possible values: security (threat intelligence), hostname (IP-Hostname lookup), liveHostname (live hostname lookup), hostnameFallbackLive (hostname with live fallback), user_agent (parse User-Agent header), abuse (abuse contact info), dma_code (DMA code), geo_accuracy (accuracy_radius, confidence, locality), * (all modules).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
}

impl GeolocationLookupV2QueryRequest {
    pub fn builder() -> GeolocationLookupV2QueryRequestBuilder {
        <GeolocationLookupV2QueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeolocationLookupV2QueryRequestBuilder {
    api_key: Option<String>,
    format: Option<GeolocationLookupV2RequestFormat>,
    ip: Option<String>,
    lang: Option<GeolocationLookupV2RequestLang>,
    fields: Option<String>,
    excludes: Option<String>,
    include: Option<String>,
}

impl GeolocationLookupV2QueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: GeolocationLookupV2RequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn lang(mut self, value: GeolocationLookupV2RequestLang) -> Self {
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

    /// Consumes the builder and constructs a [`GeolocationLookupV2QueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](GeolocationLookupV2QueryRequestBuilder::api_key)
    pub fn build(self) -> Result<GeolocationLookupV2QueryRequest, BuildError> {
        Ok(GeolocationLookupV2QueryRequest {
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
