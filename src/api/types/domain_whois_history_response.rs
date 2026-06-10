pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DomainWhoisHistoryResponse {
    /// Determines whether the request was successfully processed or not.
    #[serde(default)]
    pub status: bool,
    /// Indicates that this response contains historical data.
    pub whois: DomainWhoisHistoryResponseWhois,
    /// Shows the total number of records found for the queried domain.
    #[serde(default)]
    pub total_records: String,
    #[serde(default)]
    pub whois_domains_historical: Vec<DomainWhoisHistoryResponseWhoisDomainsHistoricalItem>,
}

impl DomainWhoisHistoryResponse {
    pub fn builder() -> DomainWhoisHistoryResponseBuilder {
        <DomainWhoisHistoryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainWhoisHistoryResponseBuilder {
    status: Option<bool>,
    whois: Option<DomainWhoisHistoryResponseWhois>,
    total_records: Option<String>,
    whois_domains_historical: Option<Vec<DomainWhoisHistoryResponseWhoisDomainsHistoricalItem>>,
}

impl DomainWhoisHistoryResponseBuilder {
    pub fn status(mut self, value: bool) -> Self {
        self.status = Some(value);
        self
    }

    pub fn whois(mut self, value: DomainWhoisHistoryResponseWhois) -> Self {
        self.whois = Some(value);
        self
    }

    pub fn total_records(mut self, value: impl Into<String>) -> Self {
        self.total_records = Some(value.into());
        self
    }

    pub fn whois_domains_historical(
        mut self,
        value: Vec<DomainWhoisHistoryResponseWhoisDomainsHistoricalItem>,
    ) -> Self {
        self.whois_domains_historical = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainWhoisHistoryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](DomainWhoisHistoryResponseBuilder::status)
    /// - [`whois`](DomainWhoisHistoryResponseBuilder::whois)
    /// - [`total_records`](DomainWhoisHistoryResponseBuilder::total_records)
    /// - [`whois_domains_historical`](DomainWhoisHistoryResponseBuilder::whois_domains_historical)
    pub fn build(self) -> Result<DomainWhoisHistoryResponse, BuildError> {
        Ok(DomainWhoisHistoryResponse {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            whois: self
                .whois
                .ok_or_else(|| BuildError::missing_field("whois"))?,
            total_records: self
                .total_records
                .ok_or_else(|| BuildError::missing_field("total_records"))?,
            whois_domains_historical: self
                .whois_domains_historical
                .ok_or_else(|| BuildError::missing_field("whois_domains_historical"))?,
        })
    }
}
