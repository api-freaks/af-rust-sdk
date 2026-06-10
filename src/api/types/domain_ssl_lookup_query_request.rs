pub use crate::prelude::*;

/// Query parameters for domain_ssl_lookup
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainSslLookupQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<DomainSslLookupRequestFormat>,
    /// Domain name or URL whose SSL certificate lookup is required
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
    /// Set to true to get the raw openSSL response of the domain
    #[serde(rename = "sslRaw")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_raw: Option<bool>,
}

impl DomainSslLookupQueryRequest {
    pub fn builder() -> DomainSslLookupQueryRequestBuilder {
        <DomainSslLookupQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainSslLookupQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<DomainSslLookupRequestFormat>,
    domain_name: Option<String>,
    ssl_raw: Option<bool>,
}

impl DomainSslLookupQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: DomainSslLookupRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn domain_name(mut self, value: impl Into<String>) -> Self {
        self.domain_name = Some(value.into());
        self
    }

    pub fn ssl_raw(mut self, value: bool) -> Self {
        self.ssl_raw = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainSslLookupQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](DomainSslLookupQueryRequestBuilder::api_key)
    /// - [`domain_name`](DomainSslLookupQueryRequestBuilder::domain_name)
    pub fn build(self) -> Result<DomainSslLookupQueryRequest, BuildError> {
        Ok(DomainSslLookupQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            domain_name: self
                .domain_name
                .ok_or_else(|| BuildError::missing_field("domain_name"))?,
            ssl_raw: self.ssl_raw,
        })
    }
}
