pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkDomainWhoisLookupV2Request {
    /// List of domain names to retrieve WHOIS data for.
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
    pub format: Option<BulkDomainWhoisLookupV2RequestFormat>,
}

impl BulkDomainWhoisLookupV2Request {
    pub fn builder() -> BulkDomainWhoisLookupV2RequestBuilder {
        <BulkDomainWhoisLookupV2RequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDomainWhoisLookupV2RequestBuilder {
    domain_names: Option<Vec<String>>,
    api_key: Option<String>,
    format: Option<BulkDomainWhoisLookupV2RequestFormat>,
}

impl BulkDomainWhoisLookupV2RequestBuilder {
    pub fn domain_names(mut self, value: Vec<String>) -> Self {
        self.domain_names = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: BulkDomainWhoisLookupV2RequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkDomainWhoisLookupV2Request`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain_names`](BulkDomainWhoisLookupV2RequestBuilder::domain_names)
    /// - [`api_key`](BulkDomainWhoisLookupV2RequestBuilder::api_key)
    pub fn build(self) -> Result<BulkDomainWhoisLookupV2Request, BuildError> {
        Ok(BulkDomainWhoisLookupV2Request {
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
