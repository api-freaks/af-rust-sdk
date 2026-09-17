pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkDomainDnsLookupRequest {
    /// List of hostnames to lookup DNS records for
    #[serde(rename = "domainNames")]
    #[serde(default)]
    pub domain_names: Vec<String>,
    /// Array of IP addresses to include in the lookup for PTR record enrichment.
    #[serde(rename = "ipAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<Vec<String>>,
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing)]
    pub format: Option<BulkDomainDnsLookupRequestFormat>,
    /// A comma-separated list of DNS record types for lookup.
    /// Possible values: A, AAAA, MX, NS, SOA, SPF, TXT, CNAME, or all
    #[serde(skip_serializing)]
    #[serde(default)]
    pub r#type: Vec<Option<String>>,
}

impl BulkDomainDnsLookupRequest {
    pub fn builder() -> BulkDomainDnsLookupRequestBuilder {
        <BulkDomainDnsLookupRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDomainDnsLookupRequestBuilder {
    domain_names: Option<Vec<String>>,
    ip_addresses: Option<Vec<String>>,
    api_key: Option<String>,
    format: Option<BulkDomainDnsLookupRequestFormat>,
    r#type: Option<Vec<Option<String>>>,
}

impl BulkDomainDnsLookupRequestBuilder {
    pub fn domain_names(mut self, value: Vec<String>) -> Self {
        self.domain_names = Some(value);
        self
    }

    pub fn ip_addresses(mut self, value: Vec<String>) -> Self {
        self.ip_addresses = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: BulkDomainDnsLookupRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn r#type(mut self, value: Vec<Option<String>>) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkDomainDnsLookupRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain_names`](BulkDomainDnsLookupRequestBuilder::domain_names)
    /// - [`api_key`](BulkDomainDnsLookupRequestBuilder::api_key)
    /// - [`r#type`](BulkDomainDnsLookupRequestBuilder::r#type)
    pub fn build(self) -> Result<BulkDomainDnsLookupRequest, BuildError> {
        Ok(BulkDomainDnsLookupRequest {
            domain_names: self
                .domain_names
                .ok_or_else(|| BuildError::missing_field("domain_names"))?,
            ip_addresses: self.ip_addresses,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
