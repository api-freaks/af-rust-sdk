pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkGeolocationLookupRequest {
    /// List of IP addresses or hostnames to lookup
    #[serde(default)]
    pub ips: Vec<String>,
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing)]
    pub format: Option<BulkGeolocationLookupRequestFormat>,
    /// Language of the response.
    #[serde(skip_serializing)]
    pub lang: Option<String>,
    /// Comma-separated list of fields to include in the response. Can include "geo".
    #[serde(skip_serializing)]
    pub fields: Option<String>,
    /// Comma-separated list of fields to exclude from the response (except "ip").
    #[serde(skip_serializing)]
    pub excludes: Option<String>,
    /// Comma-separated list of additional information to include in the response.
    #[serde(skip_serializing)]
    pub include: Option<String>,
}

impl BulkGeolocationLookupRequest {
    pub fn builder() -> BulkGeolocationLookupRequestBuilder {
        <BulkGeolocationLookupRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupRequestBuilder {
    ips: Option<Vec<String>>,
    api_key: Option<String>,
    format: Option<BulkGeolocationLookupRequestFormat>,
    lang: Option<String>,
    fields: Option<String>,
    excludes: Option<String>,
    include: Option<String>,
}

impl BulkGeolocationLookupRequestBuilder {
    pub fn ips(mut self, value: Vec<String>) -> Self {
        self.ips = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: BulkGeolocationLookupRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn lang(mut self, value: impl Into<String>) -> Self {
        self.lang = Some(value.into());
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

    /// Consumes the builder and constructs a [`BulkGeolocationLookupRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ips`](BulkGeolocationLookupRequestBuilder::ips)
    /// - [`api_key`](BulkGeolocationLookupRequestBuilder::api_key)
    pub fn build(self) -> Result<BulkGeolocationLookupRequest, BuildError> {
        Ok(BulkGeolocationLookupRequest {
            ips: self.ips.ok_or_else(|| BuildError::missing_field("ips"))?,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            lang: self.lang,
            fields: self.fields,
            excludes: self.excludes,
            include: self.include,
        })
    }
}
