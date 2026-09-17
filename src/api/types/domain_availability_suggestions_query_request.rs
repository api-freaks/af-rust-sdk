pub use crate::prelude::*;

/// Query parameters for domain_availability_suggestions
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainAvailabilitySuggestionsQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<DomainAvailabilitySuggestionsRequestFormat>,
    /// Domain name for availability and suggestions.
    #[serde(default)]
    pub domain: String,
    /// Specify the data source for domain availability checks. Use "dns" for DNS-based lookups or "whois" for WHOIS-based lookups. By default, "dns" is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<DomainAvailabilitySuggestionsRequestSource>,
    /// Number of suggestions to retrieve. The API returns a minimum of 5 suggestions regardless of a lower value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Controls the response shape. When `false`, returns a single availability object for the queried domain only. When omitted or `true`, returns an array of suggested domains instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sug: Option<bool>,
}

impl DomainAvailabilitySuggestionsQueryRequest {
    pub fn builder() -> DomainAvailabilitySuggestionsQueryRequestBuilder {
        <DomainAvailabilitySuggestionsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainAvailabilitySuggestionsQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<DomainAvailabilitySuggestionsRequestFormat>,
    domain: Option<String>,
    source: Option<DomainAvailabilitySuggestionsRequestSource>,
    count: Option<i64>,
    sug: Option<bool>,
}

impl DomainAvailabilitySuggestionsQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: DomainAvailabilitySuggestionsRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn source(mut self, value: DomainAvailabilitySuggestionsRequestSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn sug(mut self, value: bool) -> Self {
        self.sug = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainAvailabilitySuggestionsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](DomainAvailabilitySuggestionsQueryRequestBuilder::api_key)
    /// - [`domain`](DomainAvailabilitySuggestionsQueryRequestBuilder::domain)
    pub fn build(self) -> Result<DomainAvailabilitySuggestionsQueryRequest, BuildError> {
        Ok(DomainAvailabilitySuggestionsQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            domain: self
                .domain
                .ok_or_else(|| BuildError::missing_field("domain"))?,
            source: self.source,
            count: self.count,
            sug: self.sug,
        })
    }
}
