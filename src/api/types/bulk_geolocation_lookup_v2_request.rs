pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkGeolocationLookupV2Request {
    /// List of IP addresses or hostnames to lookup.
    #[serde(default)]
    pub ips: Vec<String>,
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing)]
    pub format: Option<BulkGeolocationLookupV2RequestFormat>,
    /// Response language for location fields. Default: en.
    #[serde(skip_serializing)]
    pub lang: Option<BulkGeolocationLookupV2RequestLang>,
    /// Comma-separated list of fields to include in the response. For example, `location` includes all location fields, `location.city` is a specific field.
    #[serde(skip_serializing)]
    pub fields: Option<String>,
    /// Comma-separated list of fields to exclude from response.
    #[serde(skip_serializing)]
    pub excludes: Option<String>,
    /// Comma-separated list of additional data modules to include. Possible values: security (threat intelligence), hostname (IP-Hostname lookup), liveHostname (live hostname lookup), user_agent (parse User-Agent header), abuse (abuse contact info), * (all modules).
    #[serde(skip_serializing)]
    pub include: Option<String>,
}

impl BulkGeolocationLookupV2Request {
    pub fn builder() -> BulkGeolocationLookupV2RequestBuilder {
        <BulkGeolocationLookupV2RequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupV2RequestBuilder {
    ips: Option<Vec<String>>,
    api_key: Option<String>,
    format: Option<BulkGeolocationLookupV2RequestFormat>,
    lang: Option<BulkGeolocationLookupV2RequestLang>,
    fields: Option<String>,
    excludes: Option<String>,
    include: Option<String>,
}

impl BulkGeolocationLookupV2RequestBuilder {
    pub fn ips(mut self, value: Vec<String>) -> Self {
        self.ips = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: BulkGeolocationLookupV2RequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn lang(mut self, value: BulkGeolocationLookupV2RequestLang) -> Self {
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

    /// Consumes the builder and constructs a [`BulkGeolocationLookupV2Request`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ips`](BulkGeolocationLookupV2RequestBuilder::ips)
    /// - [`api_key`](BulkGeolocationLookupV2RequestBuilder::api_key)
    pub fn build(self) -> Result<BulkGeolocationLookupV2Request, BuildError> {
        Ok(BulkGeolocationLookupV2Request {
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
