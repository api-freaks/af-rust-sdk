pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DomainDnsLookupResponse {
    /// Indicates whether the query was processed successfully.
    #[serde(default)]
    pub status: bool,
    /// Timestamp when the query was executed (format YYYY-MM-DD HH:mm:ss, not ISO 8601).
    #[serde(rename = "queryTime")]
    #[serde(default)]
    pub query_time: String,
    /// Queried domain.
    #[serde(rename = "domainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// Indicates whether the domain is registered.
    #[serde(rename = "domainRegistered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_registered: Option<bool>,
    #[serde(rename = "dnsTypes")]
    #[serde(default)]
    pub dns_types: DomainDnsLookupResponseDnsTypes,
    /// List of DNS records, each based on its type.
    #[serde(rename = "dnsRecords")]
    #[serde(default)]
    pub dns_records: Vec<DomainDnsLookupResponseDnsRecordsItem>,
    /// IP address queried (present when ipAddress parameter is used instead of host-name).
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

impl DomainDnsLookupResponse {
    pub fn builder() -> DomainDnsLookupResponseBuilder {
        <DomainDnsLookupResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainDnsLookupResponseBuilder {
    status: Option<bool>,
    query_time: Option<String>,
    domain_name: Option<String>,
    domain_registered: Option<bool>,
    dns_types: Option<DomainDnsLookupResponseDnsTypes>,
    dns_records: Option<Vec<DomainDnsLookupResponseDnsRecordsItem>>,
    ip_address: Option<String>,
}

impl DomainDnsLookupResponseBuilder {
    pub fn status(mut self, value: bool) -> Self {
        self.status = Some(value);
        self
    }

    pub fn query_time(mut self, value: impl Into<String>) -> Self {
        self.query_time = Some(value.into());
        self
    }

    pub fn domain_name(mut self, value: impl Into<String>) -> Self {
        self.domain_name = Some(value.into());
        self
    }

    pub fn domain_registered(mut self, value: bool) -> Self {
        self.domain_registered = Some(value);
        self
    }

    pub fn dns_types(mut self, value: DomainDnsLookupResponseDnsTypes) -> Self {
        self.dns_types = Some(value);
        self
    }

    pub fn dns_records(mut self, value: Vec<DomainDnsLookupResponseDnsRecordsItem>) -> Self {
        self.dns_records = Some(value);
        self
    }

    pub fn ip_address(mut self, value: impl Into<String>) -> Self {
        self.ip_address = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainDnsLookupResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](DomainDnsLookupResponseBuilder::status)
    /// - [`query_time`](DomainDnsLookupResponseBuilder::query_time)
    /// - [`dns_types`](DomainDnsLookupResponseBuilder::dns_types)
    /// - [`dns_records`](DomainDnsLookupResponseBuilder::dns_records)
    pub fn build(self) -> Result<DomainDnsLookupResponse, BuildError> {
        Ok(DomainDnsLookupResponse {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            query_time: self
                .query_time
                .ok_or_else(|| BuildError::missing_field("query_time"))?,
            domain_name: self.domain_name,
            domain_registered: self.domain_registered,
            dns_types: self
                .dns_types
                .ok_or_else(|| BuildError::missing_field("dns_types"))?,
            dns_records: self
                .dns_records
                .ok_or_else(|| BuildError::missing_field("dns_records"))?,
            ip_address: self.ip_address,
        })
    }
}
