pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DomainDnsHistoryResponseHistoricalDnsRecordsItem {
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
    pub dns_types: DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsTypes,
    #[serde(rename = "dnsRecords")]
    #[serde(default)]
    pub dns_records: Vec<DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItem>,
}

impl DomainDnsHistoryResponseHistoricalDnsRecordsItem {
    pub fn builder() -> DomainDnsHistoryResponseHistoricalDnsRecordsItemBuilder {
        <DomainDnsHistoryResponseHistoricalDnsRecordsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainDnsHistoryResponseHistoricalDnsRecordsItemBuilder {
    query_time: Option<DateTime<FixedOffset>>,
    domain_name: Option<String>,
    domain_registered: Option<bool>,
    dns_types: Option<DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsTypes>,
    dns_records: Option<Vec<DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItem>>,
}

impl DomainDnsHistoryResponseHistoricalDnsRecordsItemBuilder {
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
        value: DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsTypes,
    ) -> Self {
        self.dns_types = Some(value);
        self
    }

    pub fn dns_records(
        mut self,
        value: Vec<DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItem>,
    ) -> Self {
        self.dns_records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainDnsHistoryResponseHistoricalDnsRecordsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`query_time`](DomainDnsHistoryResponseHistoricalDnsRecordsItemBuilder::query_time)
    /// - [`domain_name`](DomainDnsHistoryResponseHistoricalDnsRecordsItemBuilder::domain_name)
    /// - [`domain_registered`](DomainDnsHistoryResponseHistoricalDnsRecordsItemBuilder::domain_registered)
    /// - [`dns_types`](DomainDnsHistoryResponseHistoricalDnsRecordsItemBuilder::dns_types)
    /// - [`dns_records`](DomainDnsHistoryResponseHistoricalDnsRecordsItemBuilder::dns_records)
    pub fn build(self) -> Result<DomainDnsHistoryResponseHistoricalDnsRecordsItem, BuildError> {
        Ok(DomainDnsHistoryResponseHistoricalDnsRecordsItem {
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
