pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DomainDnsReverseResponseReverseDnsRecordsItem {
    #[serde(rename = "queryTime")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub query_time: DateTime<FixedOffset>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "domainRegistered")]
    #[serde(default)]
    pub domain_registered: bool,
    #[serde(rename = "dnsTypes")]
    #[serde(default)]
    pub dns_types: DomainDnsReverseResponseReverseDnsRecordsItemDnsTypes,
    #[serde(rename = "dnsRecords")]
    #[serde(default)]
    pub dns_records: Vec<DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItem>,
}

impl DomainDnsReverseResponseReverseDnsRecordsItem {
    pub fn builder() -> DomainDnsReverseResponseReverseDnsRecordsItemBuilder {
        <DomainDnsReverseResponseReverseDnsRecordsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainDnsReverseResponseReverseDnsRecordsItemBuilder {
    query_time: Option<DateTime<FixedOffset>>,
    domain_name: Option<String>,
    domain_registered: Option<bool>,
    dns_types: Option<DomainDnsReverseResponseReverseDnsRecordsItemDnsTypes>,
    dns_records: Option<Vec<DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItem>>,
}

impl DomainDnsReverseResponseReverseDnsRecordsItemBuilder {
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

    pub fn dns_types(
        mut self,
        value: DomainDnsReverseResponseReverseDnsRecordsItemDnsTypes,
    ) -> Self {
        self.dns_types = Some(value);
        self
    }

    pub fn dns_records(
        mut self,
        value: Vec<DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItem>,
    ) -> Self {
        self.dns_records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainDnsReverseResponseReverseDnsRecordsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`query_time`](DomainDnsReverseResponseReverseDnsRecordsItemBuilder::query_time)
    /// - [`domain_name`](DomainDnsReverseResponseReverseDnsRecordsItemBuilder::domain_name)
    /// - [`domain_registered`](DomainDnsReverseResponseReverseDnsRecordsItemBuilder::domain_registered)
    /// - [`dns_types`](DomainDnsReverseResponseReverseDnsRecordsItemBuilder::dns_types)
    /// - [`dns_records`](DomainDnsReverseResponseReverseDnsRecordsItemBuilder::dns_records)
    pub fn build(self) -> Result<DomainDnsReverseResponseReverseDnsRecordsItem, BuildError> {
        Ok(DomainDnsReverseResponseReverseDnsRecordsItem {
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
