pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubdomainsLookupResponse {
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub status: bool,
    #[serde(default)]
    pub query_time: String,
    #[serde(default)]
    pub current_page: i64,
    #[serde(default)]
    pub total_pages: i64,
    #[serde(default)]
    pub total_records: i64,
    #[serde(default)]
    pub subdomains: Vec<SubdomainsLookupResponseSubdomainsItem>,
}

impl SubdomainsLookupResponse {
    pub fn builder() -> SubdomainsLookupResponseBuilder {
        <SubdomainsLookupResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubdomainsLookupResponseBuilder {
    domain: Option<String>,
    status: Option<bool>,
    query_time: Option<String>,
    current_page: Option<i64>,
    total_pages: Option<i64>,
    total_records: Option<i64>,
    subdomains: Option<Vec<SubdomainsLookupResponseSubdomainsItem>>,
}

impl SubdomainsLookupResponseBuilder {
    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn status(mut self, value: bool) -> Self {
        self.status = Some(value);
        self
    }

    pub fn query_time(mut self, value: impl Into<String>) -> Self {
        self.query_time = Some(value.into());
        self
    }

    pub fn current_page(mut self, value: i64) -> Self {
        self.current_page = Some(value);
        self
    }

    pub fn total_pages(mut self, value: i64) -> Self {
        self.total_pages = Some(value);
        self
    }

    pub fn total_records(mut self, value: i64) -> Self {
        self.total_records = Some(value);
        self
    }

    pub fn subdomains(mut self, value: Vec<SubdomainsLookupResponseSubdomainsItem>) -> Self {
        self.subdomains = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubdomainsLookupResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain`](SubdomainsLookupResponseBuilder::domain)
    /// - [`status`](SubdomainsLookupResponseBuilder::status)
    /// - [`query_time`](SubdomainsLookupResponseBuilder::query_time)
    /// - [`current_page`](SubdomainsLookupResponseBuilder::current_page)
    /// - [`total_pages`](SubdomainsLookupResponseBuilder::total_pages)
    /// - [`total_records`](SubdomainsLookupResponseBuilder::total_records)
    /// - [`subdomains`](SubdomainsLookupResponseBuilder::subdomains)
    pub fn build(self) -> Result<SubdomainsLookupResponse, BuildError> {
        Ok(SubdomainsLookupResponse {
            domain: self
                .domain
                .ok_or_else(|| BuildError::missing_field("domain"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            query_time: self
                .query_time
                .ok_or_else(|| BuildError::missing_field("query_time"))?,
            current_page: self
                .current_page
                .ok_or_else(|| BuildError::missing_field("current_page"))?,
            total_pages: self
                .total_pages
                .ok_or_else(|| BuildError::missing_field("total_pages"))?,
            total_records: self
                .total_records
                .ok_or_else(|| BuildError::missing_field("total_records"))?,
            subdomains: self
                .subdomains
                .ok_or_else(|| BuildError::missing_field("subdomains"))?,
        })
    }
}
