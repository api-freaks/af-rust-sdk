pub use crate::prelude::*;

/// Query parameters for subdomains_lookup
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubdomainsLookupQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<SubdomainsLookupRequestFormat>,
    /// Domain name for availability and suggestions.
    #[serde(default)]
    pub domain: String,
    /// Filter subdomains seen after this date (format YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<NaiveDate>,
    /// Filter subdomains seen before this date( format YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<NaiveDate>,
    /// Filter subdomains by status (active or inactive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SubdomainsLookupRequestStatus>,
    /// Page number for paginated results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
}

impl SubdomainsLookupQueryRequest {
    pub fn builder() -> SubdomainsLookupQueryRequestBuilder {
        <SubdomainsLookupQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubdomainsLookupQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<SubdomainsLookupRequestFormat>,
    domain: Option<String>,
    after: Option<NaiveDate>,
    before: Option<NaiveDate>,
    status: Option<SubdomainsLookupRequestStatus>,
    page: Option<String>,
}

impl SubdomainsLookupQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: SubdomainsLookupRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn after(mut self, value: NaiveDate) -> Self {
        self.after = Some(value);
        self
    }

    pub fn before(mut self, value: NaiveDate) -> Self {
        self.before = Some(value);
        self
    }

    pub fn status(mut self, value: SubdomainsLookupRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn page(mut self, value: impl Into<String>) -> Self {
        self.page = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubdomainsLookupQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](SubdomainsLookupQueryRequestBuilder::api_key)
    /// - [`domain`](SubdomainsLookupQueryRequestBuilder::domain)
    pub fn build(self) -> Result<SubdomainsLookupQueryRequest, BuildError> {
        Ok(SubdomainsLookupQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            domain: self
                .domain
                .ok_or_else(|| BuildError::missing_field("domain"))?,
            after: self.after,
            before: self.before,
            status: self.status,
            page: self.page,
        })
    }
}
