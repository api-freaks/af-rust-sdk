pub use crate::prelude::*;

/// Query parameters for domain_reputation
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainReputationQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<DomainReputationRequestFormat>,
    /// The domain name to assess (e.g. example.com). Must contain at least one dot and be at most 253 characters. Automatically lowercased.
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
}

impl DomainReputationQueryRequest {
    pub fn builder() -> DomainReputationQueryRequestBuilder {
        <DomainReputationQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainReputationQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<DomainReputationRequestFormat>,
    domain_name: Option<String>,
}

impl DomainReputationQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: DomainReputationRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn domain_name(mut self, value: impl Into<String>) -> Self {
        self.domain_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainReputationQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](DomainReputationQueryRequestBuilder::api_key)
    /// - [`domain_name`](DomainReputationQueryRequestBuilder::domain_name)
    pub fn build(self) -> Result<DomainReputationQueryRequest, BuildError> {
        Ok(DomainReputationQueryRequest {
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
