pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DomainDnsHistoryResponse {
    #[serde(rename = "totalRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_records: Option<i64>,
    #[serde(rename = "totalPages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<i64>,
    #[serde(rename = "currentPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_page: Option<i64>,
    #[serde(rename = "historicalDnsRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical_dns_records: Option<Vec<DomainDnsHistoryResponseHistoricalDnsRecordsItem>>,
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
    pub fn build(self) -> Result<DomainDnsHistoryResponse, BuildError> {
        Ok(DomainDnsHistoryResponse {
            total_records: self.total_records,
            total_pages: self.total_pages,
            current_page: self.current_page,
            historical_dns_records: self.historical_dns_records,
        })
    }
}
