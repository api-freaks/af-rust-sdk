pub use crate::prelude::*;

/// Query parameters for domain_dns_reverse
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DomainDnsReverseQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<DomainDnsReverseRequestFormat>,
    /// The type of reverse DNS lookup to perform. Determines how the value parameter is interpreted:
    /// - A: IPv4 CIDR block
    /// - AAAA: IPv6 CIDR block
    /// - MX: Mail provider domain
    /// - NS: Name server provider hostname
    /// - SOA: SOA record admin domain
    /// - SPF/TXT: Target verification strings
    /// - CNAME: Target hostname
    pub r#type: DomainDnsReverseRequestType,
    /// Provide an IP or CIDR for A/AAAA lookups, or a hostname/selector for MX, NS, SOA, SPF, TXT, and CNAME queries. Wildcard regex patterns are also supported (e.g., mail.google.com, m*.google.com, _spf.g*.com, s*.g*.com).
    #[serde(default)]
    pub value: String,
    /// Accepts 'true' or 'false'. "true" returns only records that exactly match the input (NS, MX, CNAME, SOA, SPF, TXT). "false" returns all matches (default when omitted).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<bool>,
    /// Page number to paginate through results (defaults to 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl DomainDnsReverseQueryRequest {
    pub fn builder() -> DomainDnsReverseQueryRequestBuilder {
        <DomainDnsReverseQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainDnsReverseQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<DomainDnsReverseRequestFormat>,
    r#type: Option<DomainDnsReverseRequestType>,
    value: Option<String>,
    exact: Option<bool>,
    page: Option<i64>,
}

impl DomainDnsReverseQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: DomainDnsReverseRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn r#type(mut self, value: DomainDnsReverseRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn exact(mut self, value: bool) -> Self {
        self.exact = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainDnsReverseQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](DomainDnsReverseQueryRequestBuilder::api_key)
    /// - [`r#type`](DomainDnsReverseQueryRequestBuilder::r#type)
    /// - [`value`](DomainDnsReverseQueryRequestBuilder::value)
    pub fn build(self) -> Result<DomainDnsReverseQueryRequest, BuildError> {
        Ok(DomainDnsReverseQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
            exact: self.exact,
            page: self.page,
        })
    }
}
