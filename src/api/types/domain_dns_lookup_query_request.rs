pub use crate::prelude::*;

/// Query parameters for domain_dns_lookup
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainDnsLookupQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<DomainDnsLookupRequestFormat>,
    /// Hostname or URL whose DNS records are required.
    #[serde(rename = "host-name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    /// The IP address for requested DNS's PTR record. 'type' parameter must be set to 'all'.
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// A comma-separated list of DNS record types for lookup. Possible values: A, AAAA, MX, NS, SOA, SPF, TXT, CNAME, or all. When ipAddress is provided, type must be "all".
    #[serde(default)]
    pub r#type: Vec<Option<String>>,
}

impl DomainDnsLookupQueryRequest {
    pub fn builder() -> DomainDnsLookupQueryRequestBuilder {
        <DomainDnsLookupQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainDnsLookupQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<DomainDnsLookupRequestFormat>,
    host_name: Option<String>,
    ip_address: Option<String>,
    r#type: Option<Vec<Option<String>>>,
}

impl DomainDnsLookupQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: DomainDnsLookupRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn host_name(mut self, value: impl Into<String>) -> Self {
        self.host_name = Some(value.into());
        self
    }

    pub fn ip_address(mut self, value: impl Into<String>) -> Self {
        self.ip_address = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: Vec<Option<String>>) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainDnsLookupQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](DomainDnsLookupQueryRequestBuilder::api_key)
    /// - [`r#type`](DomainDnsLookupQueryRequestBuilder::r#type)
    pub fn build(self) -> Result<DomainDnsLookupQueryRequest, BuildError> {
        Ok(DomainDnsLookupQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            host_name: self.host_name,
            ip_address: self.ip_address,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
