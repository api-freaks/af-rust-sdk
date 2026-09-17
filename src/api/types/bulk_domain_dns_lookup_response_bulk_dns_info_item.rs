pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkDomainDnsLookupResponseBulkDnsInfoItem {
    /// Indicates whether the query was processed successfully.
    #[serde(default)]
    pub status: bool,
    /// Timestamp when the query was executed (format YYYY-MM-DD HH:mm:ss, not ISO 8601).
    #[serde(rename = "queryTime")]
    #[serde(default)]
    pub query_time: String,
    /// Queried domain. Absent when this result is for a queried IP address instead (see `ipAddress`).
    #[serde(rename = "domainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// Indicates whether the domain is registered. Absent when this result is for a queried IP address instead.
    #[serde(rename = "domainRegistered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_registered: Option<bool>,
    /// Queried IP address, present when this result is for reverse DNS (PTR) enrichment instead of a domain name.
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "dnsTypes")]
    #[serde(default)]
    pub dns_types: BulkDomainDnsLookupResponseBulkDnsInfoItemDnsTypes,
    /// List of DNS records, each based on its type.
    #[serde(rename = "dnsRecords")]
    #[serde(default)]
    pub dns_records: Vec<BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItem>,
}

impl BulkDomainDnsLookupResponseBulkDnsInfoItem {
    pub fn builder() -> BulkDomainDnsLookupResponseBulkDnsInfoItemBuilder {
        <BulkDomainDnsLookupResponseBulkDnsInfoItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDomainDnsLookupResponseBulkDnsInfoItemBuilder {
    status: Option<bool>,
    query_time: Option<String>,
    domain_name: Option<String>,
    domain_registered: Option<bool>,
    ip_address: Option<String>,
    dns_types: Option<BulkDomainDnsLookupResponseBulkDnsInfoItemDnsTypes>,
    dns_records: Option<Vec<BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItem>>,
}

impl BulkDomainDnsLookupResponseBulkDnsInfoItemBuilder {
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

    pub fn ip_address(mut self, value: impl Into<String>) -> Self {
        self.ip_address = Some(value.into());
        self
    }

    pub fn dns_types(mut self, value: BulkDomainDnsLookupResponseBulkDnsInfoItemDnsTypes) -> Self {
        self.dns_types = Some(value);
        self
    }

    pub fn dns_records(
        mut self,
        value: Vec<BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItem>,
    ) -> Self {
        self.dns_records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkDomainDnsLookupResponseBulkDnsInfoItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](BulkDomainDnsLookupResponseBulkDnsInfoItemBuilder::status)
    /// - [`query_time`](BulkDomainDnsLookupResponseBulkDnsInfoItemBuilder::query_time)
    /// - [`dns_types`](BulkDomainDnsLookupResponseBulkDnsInfoItemBuilder::dns_types)
    /// - [`dns_records`](BulkDomainDnsLookupResponseBulkDnsInfoItemBuilder::dns_records)
    pub fn build(self) -> Result<BulkDomainDnsLookupResponseBulkDnsInfoItem, BuildError> {
        Ok(BulkDomainDnsLookupResponseBulkDnsInfoItem {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            query_time: self
                .query_time
                .ok_or_else(|| BuildError::missing_field("query_time"))?,
            domain_name: self.domain_name,
            domain_registered: self.domain_registered,
            ip_address: self.ip_address,
            dns_types: self
                .dns_types
                .ok_or_else(|| BuildError::missing_field("dns_types"))?,
            dns_records: self
                .dns_records
                .ok_or_else(|| BuildError::missing_field("dns_records"))?,
        })
    }
}
