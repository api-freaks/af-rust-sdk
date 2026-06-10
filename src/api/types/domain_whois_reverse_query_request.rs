pub use crate::prelude::*;

/// Query parameters for domain_whois_reverse
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainWhoisReverseQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<DomainWhoisReverseRequestFormat>,
    /// Keyword search term for reverse WHOIS by keyword (case-insensitive pattern matching).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    /// Email search term for reverse WHOIS by email address (case-insensitive exact or regex match; * wildcard supported).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Registrant or owner name for reverse WHOIS (a full-text search phrase matching technique to retrieve results).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// Organization or company name for reverse WHOIS (full-text search phrase matching technique to retrieve results).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// Accepts 'true' or 'false'. "true" returns only records that exactly match the input (keyword, owner/registrant, or company). "false" returns all matches and is the default when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<DomainWhoisReverseRequestMode>,
    /// Page number for paginated results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl DomainWhoisReverseQueryRequest {
    pub fn builder() -> DomainWhoisReverseQueryRequestBuilder {
        <DomainWhoisReverseQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainWhoisReverseQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<DomainWhoisReverseRequestFormat>,
    keyword: Option<String>,
    email: Option<String>,
    owner: Option<String>,
    company: Option<String>,
    exact: Option<bool>,
    mode: Option<DomainWhoisReverseRequestMode>,
    page: Option<i64>,
}

impl DomainWhoisReverseQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: DomainWhoisReverseRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn keyword(mut self, value: impl Into<String>) -> Self {
        self.keyword = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn owner(mut self, value: impl Into<String>) -> Self {
        self.owner = Some(value.into());
        self
    }

    pub fn company(mut self, value: impl Into<String>) -> Self {
        self.company = Some(value.into());
        self
    }

    pub fn exact(mut self, value: bool) -> Self {
        self.exact = Some(value);
        self
    }

    pub fn mode(mut self, value: DomainWhoisReverseRequestMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainWhoisReverseQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](DomainWhoisReverseQueryRequestBuilder::api_key)
    pub fn build(self) -> Result<DomainWhoisReverseQueryRequest, BuildError> {
        Ok(DomainWhoisReverseQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            keyword: self.keyword,
            email: self.email,
            owner: self.owner,
            company: self.company,
            exact: self.exact,
            mode: self.mode,
            page: self.page,
        })
    }
}
