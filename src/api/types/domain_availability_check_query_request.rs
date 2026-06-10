pub use crate::prelude::*;

/// Query parameters for domain_availability_check
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainAvailabilityCheckQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<DomainAvailabilityCheckRequestFormat>,
    /// Domain name whose availability is to be checked.
    #[serde(default)]
    pub domain: String,
    /// Specify the data source for domain availability checks. Use "dns" for DNS-based lookups or "whois" for WHOIS-based lookups. By default, "dns" is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<DomainAvailabilityCheckRequestSource>,
}

impl DomainAvailabilityCheckQueryRequest {
    pub fn builder() -> DomainAvailabilityCheckQueryRequestBuilder {
        <DomainAvailabilityCheckQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainAvailabilityCheckQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<DomainAvailabilityCheckRequestFormat>,
    domain: Option<String>,
    source: Option<DomainAvailabilityCheckRequestSource>,
}

impl DomainAvailabilityCheckQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: DomainAvailabilityCheckRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn source(mut self, value: DomainAvailabilityCheckRequestSource) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainAvailabilityCheckQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](DomainAvailabilityCheckQueryRequestBuilder::api_key)
    /// - [`domain`](DomainAvailabilityCheckQueryRequestBuilder::domain)
    pub fn build(self) -> Result<DomainAvailabilityCheckQueryRequest, BuildError> {
        Ok(DomainAvailabilityCheckQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            domain: self
                .domain
                .ok_or_else(|| BuildError::missing_field("domain"))?,
            source: self.source,
        })
    }
}
