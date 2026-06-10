pub use crate::prelude::*;

/// Query parameters for domain_ssl_chain_lookup
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainSslChainLookupQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<DomainSslChainLookupRequestFormat>,
    /// Domain name or URL whose SSL certificate chain lookup is required
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
    /// Set to true to get the raw openSSL response for each certificate in the chain
    #[serde(rename = "sslRaw")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_raw: Option<bool>,
}

impl DomainSslChainLookupQueryRequest {
    pub fn builder() -> DomainSslChainLookupQueryRequestBuilder {
        <DomainSslChainLookupQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainSslChainLookupQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<DomainSslChainLookupRequestFormat>,
    domain_name: Option<String>,
    ssl_raw: Option<bool>,
}

impl DomainSslChainLookupQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: DomainSslChainLookupRequestFormat) -> Self {
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

    /// Consumes the builder and constructs a [`DomainSslChainLookupQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](DomainSslChainLookupQueryRequestBuilder::api_key)
    /// - [`domain_name`](DomainSslChainLookupQueryRequestBuilder::domain_name)
    pub fn build(self) -> Result<DomainSslChainLookupQueryRequest, BuildError> {
        Ok(DomainSslChainLookupQueryRequest {
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
