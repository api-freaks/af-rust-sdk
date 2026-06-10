pub use crate::prelude::*;

/// Query parameters for domain_dns_history
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainDnsHistoryQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<DomainDnsHistoryRequestFormat>,
    /// Hostname or URL whose historical DNS records are required
    #[serde(rename = "host-name")]
    #[serde(default)]
    pub host_name: String,
    /// A comma-separated list of DNS record types for lookup.
    /// Possible values: A, AAAA, MX, NS, SOA, SPF, TXT, CNAME, or all
    #[serde(default)]
    pub r#type: Vec<Option<String>>,
    /// Page number for paginated results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl DomainDnsHistoryQueryRequest {
    pub fn builder() -> DomainDnsHistoryQueryRequestBuilder {
        <DomainDnsHistoryQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainDnsHistoryQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<DomainDnsHistoryRequestFormat>,
    host_name: Option<String>,
    r#type: Option<Vec<Option<String>>>,
    page: Option<i64>,
}

impl DomainDnsHistoryQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: DomainDnsHistoryRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn host_name(mut self, value: impl Into<String>) -> Self {
        self.host_name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: Vec<Option<String>>) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainDnsHistoryQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](DomainDnsHistoryQueryRequestBuilder::api_key)
    /// - [`host_name`](DomainDnsHistoryQueryRequestBuilder::host_name)
    /// - [`r#type`](DomainDnsHistoryQueryRequestBuilder::r#type)
    pub fn build(self) -> Result<DomainDnsHistoryQueryRequest, BuildError> {
        Ok(DomainDnsHistoryQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            host_name: self
                .host_name
                .ok_or_else(|| BuildError::missing_field("host_name"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            page: self.page,
        })
    }
}
