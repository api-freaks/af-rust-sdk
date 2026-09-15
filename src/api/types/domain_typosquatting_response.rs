pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainTyposquattingResponse {
    #[serde(default)]
    pub status: bool,
    #[serde(rename = "totalRecords")]
    #[serde(default)]
    pub total_records: i64,
    #[serde(rename = "currentPage")]
    #[serde(default)]
    pub current_page: i64,
    #[serde(rename = "hasNextPage")]
    #[serde(default)]
    pub has_next_page: bool,
    #[serde(rename = "totalPages")]
    #[serde(default)]
    pub total_pages: i64,
    /// Opaque token to pass as pageToken on the next request. Present only when hasNextPage is true.
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(default)]
    pub domains: Vec<DomainTyposquattingResponseDomainsItem>,
}

impl DomainTyposquattingResponse {
    pub fn builder() -> DomainTyposquattingResponseBuilder {
        <DomainTyposquattingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainTyposquattingResponseBuilder {
    status: Option<bool>,
    total_records: Option<i64>,
    current_page: Option<i64>,
    has_next_page: Option<bool>,
    total_pages: Option<i64>,
    next_page_token: Option<String>,
    domains: Option<Vec<DomainTyposquattingResponseDomainsItem>>,
}

impl DomainTyposquattingResponseBuilder {
    pub fn status(mut self, value: bool) -> Self {
        self.status = Some(value);
        self
    }

    pub fn total_records(mut self, value: i64) -> Self {
        self.total_records = Some(value);
        self
    }

    pub fn current_page(mut self, value: i64) -> Self {
        self.current_page = Some(value);
        self
    }

    pub fn has_next_page(mut self, value: bool) -> Self {
        self.has_next_page = Some(value);
        self
    }

    pub fn total_pages(mut self, value: i64) -> Self {
        self.total_pages = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());
        self
    }

    pub fn domains(mut self, value: Vec<DomainTyposquattingResponseDomainsItem>) -> Self {
        self.domains = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainTyposquattingResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](DomainTyposquattingResponseBuilder::status)
    /// - [`total_records`](DomainTyposquattingResponseBuilder::total_records)
    /// - [`current_page`](DomainTyposquattingResponseBuilder::current_page)
    /// - [`has_next_page`](DomainTyposquattingResponseBuilder::has_next_page)
    /// - [`total_pages`](DomainTyposquattingResponseBuilder::total_pages)
    /// - [`domains`](DomainTyposquattingResponseBuilder::domains)
    pub fn build(self) -> Result<DomainTyposquattingResponse, BuildError> {
        Ok(DomainTyposquattingResponse {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            total_records: self
                .total_records
                .ok_or_else(|| BuildError::missing_field("total_records"))?,
            current_page: self
                .current_page
                .ok_or_else(|| BuildError::missing_field("current_page"))?,
            has_next_page: self
                .has_next_page
                .ok_or_else(|| BuildError::missing_field("has_next_page"))?,
            total_pages: self
                .total_pages
                .ok_or_else(|| BuildError::missing_field("total_pages"))?,
            next_page_token: self.next_page_token,
            domains: self
                .domains
                .ok_or_else(|| BuildError::missing_field("domains"))?,
        })
    }
}
