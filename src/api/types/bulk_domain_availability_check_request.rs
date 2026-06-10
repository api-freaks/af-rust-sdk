pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkDomainAvailabilityCheckRequest {
    /// List of domain names to check.
    #[serde(rename = "domainNames")]
    #[serde(default)]
    pub domain_names: Vec<String>,
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing)]
    pub format: Option<BulkDomainAvailabilityCheckRequestFormat>,
    /// Specify the data source for domain availability checks. Use "dns" for DNS-based lookups or "whois" for WHOIS-based lookups. By default, "dns" is used.
    #[serde(skip_serializing)]
    pub source: Option<BulkDomainAvailabilityCheckRequestSource>,
}

impl BulkDomainAvailabilityCheckRequest {
    pub fn builder() -> BulkDomainAvailabilityCheckRequestBuilder {
        <BulkDomainAvailabilityCheckRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDomainAvailabilityCheckRequestBuilder {
    domain_names: Option<Vec<String>>,
    api_key: Option<String>,
    format: Option<BulkDomainAvailabilityCheckRequestFormat>,
    source: Option<BulkDomainAvailabilityCheckRequestSource>,
}

impl BulkDomainAvailabilityCheckRequestBuilder {
    pub fn domain_names(mut self, value: Vec<String>) -> Self {
        self.domain_names = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: BulkDomainAvailabilityCheckRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn source(mut self, value: BulkDomainAvailabilityCheckRequestSource) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkDomainAvailabilityCheckRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain_names`](BulkDomainAvailabilityCheckRequestBuilder::domain_names)
    /// - [`api_key`](BulkDomainAvailabilityCheckRequestBuilder::api_key)
    pub fn build(self) -> Result<BulkDomainAvailabilityCheckRequest, BuildError> {
        Ok(BulkDomainAvailabilityCheckRequest {
            domain_names: self
                .domain_names
                .ok_or_else(|| BuildError::missing_field("domain_names"))?,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            source: self.source,
        })
    }
}
