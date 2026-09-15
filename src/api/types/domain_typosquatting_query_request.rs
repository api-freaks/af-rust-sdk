pub use crate::prelude::*;

/// Query parameters for domain_typosquatting
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainTyposquattingQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<DomainTyposquattingRequestFormat>,
    /// Brand or label to find typo variants for. 3-63 characters, letters, digits, or hyphens, a single label with no dots. Case-insensitive. Use either keyword or pattern, never both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    /// Wildcard search string that combines fuzzy matching with * wildcards. 3-63 characters total, * is the only supported wildcard and each one matches zero or more characters, maximum 3 asterisks per request. Use either keyword or pattern, never both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    /// Token from nextPageToken in the previous response. Required to retrieve page 2 and onward. The original keyword or pattern must be passed alongside the token on every page request. Results page at 100 domains per page.
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl DomainTyposquattingQueryRequest {
    pub fn builder() -> DomainTyposquattingQueryRequestBuilder {
        <DomainTyposquattingQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainTyposquattingQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<DomainTyposquattingRequestFormat>,
    keyword: Option<String>,
    pattern: Option<String>,
    page_token: Option<String>,
}

impl DomainTyposquattingQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: DomainTyposquattingRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn keyword(mut self, value: impl Into<String>) -> Self {
        self.keyword = Some(value.into());
        self
    }

    pub fn pattern(mut self, value: impl Into<String>) -> Self {
        self.pattern = Some(value.into());
        self
    }

    pub fn page_token(mut self, value: impl Into<String>) -> Self {
        self.page_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainTyposquattingQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](DomainTyposquattingQueryRequestBuilder::api_key)
    pub fn build(self) -> Result<DomainTyposquattingQueryRequest, BuildError> {
        Ok(DomainTyposquattingQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            keyword: self.keyword,
            pattern: self.pattern,
            page_token: self.page_token,
        })
    }
}
