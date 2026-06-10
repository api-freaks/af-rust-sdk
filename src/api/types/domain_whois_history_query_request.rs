pub use crate::prelude::*;

/// Query parameters for domain_whois_history
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainWhoisHistoryQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<DomainWhoisHistoryRequestFormat>,
    /// Domain name for historical WHOIS lookup
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
}

impl DomainWhoisHistoryQueryRequest {
    pub fn builder() -> DomainWhoisHistoryQueryRequestBuilder {
        <DomainWhoisHistoryQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainWhoisHistoryQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<DomainWhoisHistoryRequestFormat>,
    domain_name: Option<String>,
}

impl DomainWhoisHistoryQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: DomainWhoisHistoryRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn domain_name(mut self, value: impl Into<String>) -> Self {
        self.domain_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainWhoisHistoryQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](DomainWhoisHistoryQueryRequestBuilder::api_key)
    /// - [`domain_name`](DomainWhoisHistoryQueryRequestBuilder::domain_name)
    pub fn build(self) -> Result<DomainWhoisHistoryQueryRequest, BuildError> {
        Ok(DomainWhoisHistoryQueryRequest {
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
