pub use crate::prelude::*;

/// Query parameters for domain_whois_lookup_v2
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainWhoisLookupV2QueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<DomainWhoisLookupV2RequestFormat>,
    /// Domain name to retrieve WHOIS data for (e.g. example.com).
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
}

impl DomainWhoisLookupV2QueryRequest {
    pub fn builder() -> DomainWhoisLookupV2QueryRequestBuilder {
        <DomainWhoisLookupV2QueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainWhoisLookupV2QueryRequestBuilder {
    api_key: Option<String>,
    format: Option<DomainWhoisLookupV2RequestFormat>,
    domain_name: Option<String>,
}

impl DomainWhoisLookupV2QueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: DomainWhoisLookupV2RequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn domain_name(mut self, value: impl Into<String>) -> Self {
        self.domain_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainWhoisLookupV2QueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](DomainWhoisLookupV2QueryRequestBuilder::api_key)
    /// - [`domain_name`](DomainWhoisLookupV2QueryRequestBuilder::domain_name)
    pub fn build(self) -> Result<DomainWhoisLookupV2QueryRequest, BuildError> {
        Ok(DomainWhoisLookupV2QueryRequest {
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
