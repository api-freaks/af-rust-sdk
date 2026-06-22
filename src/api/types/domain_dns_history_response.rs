pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DomainDnsHistoryResponse {
    #[serde(rename = "totalRecords")]
    #[serde(default)]
    pub total_records: i64,
    #[serde(rename = "totalPages")]
    #[serde(default)]
    pub total_pages: i64,
    #[serde(rename = "currentPage")]
    #[serde(default)]
    pub current_page: i64,
    #[serde(rename = "historicalDnsRecords")]
    #[serde(default)]
    pub historical_dns_records: Vec<DomainDnsHistoryResponseHistoricalDnsRecordsItem>,
}

impl DomainDnsHistoryResponse {
    pub fn builder() -> DomainDnsHistoryResponseBuilder {
        <DomainDnsHistoryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainDnsHistoryResponseBuilder {
    total_records: Option<i64>,
    total_pages: Option<i64>,
    current_page: Option<i64>,
    historical_dns_records: Option<Vec<DomainDnsHistoryResponseHistoricalDnsRecordsItem>>,
}

impl DomainDnsHistoryResponseBuilder {
    pub fn total_records(mut self, value: i64) -> Self {
        self.total_records = Some(value);
        self
    }

    pub fn total_pages(mut self, value: i64) -> Self {
        self.total_pages = Some(value);
        self
    }

    pub fn current_page(mut self, value: i64) -> Self {
        self.current_page = Some(value);
        self
    }

    pub fn historical_dns_records(
        mut self,
        value: Vec<DomainDnsHistoryResponseHistoricalDnsRecordsItem>,
    ) -> Self {
        self.historical_dns_records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainDnsHistoryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total_records`](DomainDnsHistoryResponseBuilder::total_records)
    /// - [`total_pages`](DomainDnsHistoryResponseBuilder::total_pages)
    /// - [`current_page`](DomainDnsHistoryResponseBuilder::current_page)
    /// - [`historical_dns_records`](DomainDnsHistoryResponseBuilder::historical_dns_records)
    pub fn build(self) -> Result<DomainDnsHistoryResponse, BuildError> {
        Ok(DomainDnsHistoryResponse {
            total_records: self
                .total_records
                .ok_or_else(|| BuildError::missing_field("total_records"))?,
            total_pages: self
                .total_pages
                .ok_or_else(|| BuildError::missing_field("total_pages"))?,
            current_page: self
                .current_page
                .ok_or_else(|| BuildError::missing_field("current_page"))?,
            historical_dns_records: self
                .historical_dns_records
                .ok_or_else(|| BuildError::missing_field("historical_dns_records"))?,
        })
    }
}
