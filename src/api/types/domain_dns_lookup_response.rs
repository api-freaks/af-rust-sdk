pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DomainDnsLookupResponse {
    /// Indicates whether the query was processed successfully.
    #[serde(default)]
    pub status: bool,
    /// Time at which the query was made (Format:YYYY-MM-DD HH:mm:ss).
    #[serde(rename = "queryTime")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub query_time: DateTime<FixedOffset>,
    /// Queried domain.
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
    /// Indicates whether the domain is registered.
    #[serde(rename = "domainRegistered")]
    #[serde(default)]
    pub domain_registered: bool,
    #[serde(rename = "dnsTypes")]
    #[serde(default)]
    pub dns_types: DomainDnsLookupResponseDnsTypes,
    /// List of DNS records, each based on its type.
    #[serde(rename = "dnsRecords")]
    #[serde(default)]
    pub dns_records: Vec<DomainDnsLookupResponseDnsRecordsItem>,
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
    query_time: Option<DateTime<FixedOffset>>,
    domain_name: Option<String>,
    domain_registered: Option<bool>,
    dns_types: Option<DomainDnsLookupResponseDnsTypes>,
    dns_records: Option<Vec<DomainDnsLookupResponseDnsRecordsItem>>,
}

impl DomainDnsLookupResponseBuilder {
    pub fn status(mut self, value: bool) -> Self {
        self.status = Some(value);
        self
    }

    pub fn query_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.query_time = Some(value);
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

    /// Consumes the builder and constructs a [`DomainDnsLookupResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](DomainDnsLookupResponseBuilder::status)
    /// - [`query_time`](DomainDnsLookupResponseBuilder::query_time)
    /// - [`domain_name`](DomainDnsLookupResponseBuilder::domain_name)
    /// - [`domain_registered`](DomainDnsLookupResponseBuilder::domain_registered)
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
            domain_name: self
                .domain_name
                .ok_or_else(|| BuildError::missing_field("domain_name"))?,
            domain_registered: self
                .domain_registered
                .ok_or_else(|| BuildError::missing_field("domain_registered"))?,
            dns_types: self
                .dns_types
                .ok_or_else(|| BuildError::missing_field("dns_types"))?,
            dns_records: self
                .dns_records
                .ok_or_else(|| BuildError::missing_field("dns_records"))?,
        })
    }
}
