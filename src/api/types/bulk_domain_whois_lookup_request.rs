pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkDomainWhoisLookupRequest {
    /// A list of domain names for which WHOIS data is requested.
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
    pub format: Option<BulkDomainWhoisLookupRequestFormat>,
}

impl BulkDomainWhoisLookupRequest {
    pub fn builder() -> BulkDomainWhoisLookupRequestBuilder {
        <BulkDomainWhoisLookupRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDomainWhoisLookupRequestBuilder {
    domain_names: Option<Vec<String>>,
    api_key: Option<String>,
    format: Option<BulkDomainWhoisLookupRequestFormat>,
}

impl BulkDomainWhoisLookupRequestBuilder {
    pub fn domain_names(mut self, value: Vec<String>) -> Self {
        self.domain_names = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: BulkDomainWhoisLookupRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkDomainWhoisLookupRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain_names`](BulkDomainWhoisLookupRequestBuilder::domain_names)
    /// - [`api_key`](BulkDomainWhoisLookupRequestBuilder::api_key)
    pub fn build(self) -> Result<BulkDomainWhoisLookupRequest, BuildError> {
        Ok(BulkDomainWhoisLookupRequest {
            domain_names: self
                .domain_names
                .ok_or_else(|| BuildError::missing_field("domain_names"))?,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
        })
    }
}
