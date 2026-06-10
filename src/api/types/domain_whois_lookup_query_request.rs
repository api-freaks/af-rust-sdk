pub use crate::prelude::*;

/// Query parameters for domain_whois_lookup
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainWhoisLookupQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Response format (defaults to json)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<DomainWhoisLookupRequestFormat>,
    /// Domain name for WHOIS lookup
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
}

impl DomainWhoisLookupQueryRequest {
    pub fn builder() -> DomainWhoisLookupQueryRequestBuilder {
        <DomainWhoisLookupQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainWhoisLookupQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<DomainWhoisLookupRequestFormat>,
    domain_name: Option<String>,
}

impl DomainWhoisLookupQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: DomainWhoisLookupRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn domain_name(mut self, value: impl Into<String>) -> Self {
        self.domain_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainWhoisLookupQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](DomainWhoisLookupQueryRequestBuilder::api_key)
    /// - [`domain_name`](DomainWhoisLookupQueryRequestBuilder::domain_name)
    pub fn build(self) -> Result<DomainWhoisLookupQueryRequest, BuildError> {
        Ok(DomainWhoisLookupQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            domain_name: self
                .domain_name
                .ok_or_else(|| BuildError::missing_field("domain_name"))?,
        })
    }
}
