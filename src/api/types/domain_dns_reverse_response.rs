pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DomainDnsReverseResponse {
    #[serde(rename = "totalRecords")]
    #[serde(default)]
    pub total_records: i64,
    #[serde(rename = "totalPages")]
    #[serde(default)]
    pub total_pages: i64,
    #[serde(rename = "currentPage")]
    #[serde(default)]
    pub current_page: i64,
    #[serde(rename = "reverseDnsRecords")]
    #[serde(default)]
    pub reverse_dns_records: Vec<DomainDnsReverseResponseReverseDnsRecordsItem>,
}

impl DomainDnsReverseResponse {
    pub fn builder() -> DomainDnsReverseResponseBuilder {
        <DomainDnsReverseResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainDnsReverseResponseBuilder {
    total_records: Option<i64>,
    total_pages: Option<i64>,
    current_page: Option<i64>,
    reverse_dns_records: Option<Vec<DomainDnsReverseResponseReverseDnsRecordsItem>>,
}

impl DomainDnsReverseResponseBuilder {
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

    pub fn reverse_dns_records(
        mut self,
        value: Vec<DomainDnsReverseResponseReverseDnsRecordsItem>,
    ) -> Self {
        self.reverse_dns_records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainDnsReverseResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total_records`](DomainDnsReverseResponseBuilder::total_records)
    /// - [`total_pages`](DomainDnsReverseResponseBuilder::total_pages)
    /// - [`current_page`](DomainDnsReverseResponseBuilder::current_page)
    /// - [`reverse_dns_records`](DomainDnsReverseResponseBuilder::reverse_dns_records)
    pub fn build(self) -> Result<DomainDnsReverseResponse, BuildError> {
        Ok(DomainDnsReverseResponse {
            total_records: self
                .total_records
                .ok_or_else(|| BuildError::missing_field("total_records"))?,
            total_pages: self
                .total_pages
                .ok_or_else(|| BuildError::missing_field("total_pages"))?,
            current_page: self
                .current_page
                .ok_or_else(|| BuildError::missing_field("current_page"))?,
            reverse_dns_records: self
                .reverse_dns_records
                .ok_or_else(|| BuildError::missing_field("reverse_dns_records"))?,
        })
    }
}
