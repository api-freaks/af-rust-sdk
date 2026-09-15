pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainWhoisReverseResponse {
    #[serde(rename = "totalResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_result: Option<i64>,
    #[serde(rename = "totalPages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<i64>,
    #[serde(rename = "currentPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whois_domains_historical: Option<Vec<DomainWhoisReverseResponseWhoisDomainsHistoricalItem>>,
}

impl DomainWhoisReverseResponse {
    pub fn builder() -> DomainWhoisReverseResponseBuilder {
        <DomainWhoisReverseResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainWhoisReverseResponseBuilder {
    total_result: Option<i64>,
    total_pages: Option<i64>,
    current_page: Option<i64>,
    whois_domains_historical: Option<Vec<DomainWhoisReverseResponseWhoisDomainsHistoricalItem>>,
}

impl DomainWhoisReverseResponseBuilder {
    pub fn total_result(mut self, value: i64) -> Self {
        self.total_result = Some(value);
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

    pub fn whois_domains_historical(
        mut self,
        value: Vec<DomainWhoisReverseResponseWhoisDomainsHistoricalItem>,
    ) -> Self {
        self.whois_domains_historical = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainWhoisReverseResponse`].
    pub fn build(self) -> Result<DomainWhoisReverseResponse, BuildError> {
        Ok(DomainWhoisReverseResponse {
            total_result: self.total_result,
            total_pages: self.total_pages,
            current_page: self.current_page,
            whois_domains_historical: self.whois_domains_historical,
        })
    }
}
