pub use crate::prelude::*;

/// Per-item error, returned in place of a WHOIS result when an individual domain's extension is unsupported or its lookup otherwise fails.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemError {
    /// Always false for a per-item error.
    #[serde(default)]
    pub status: bool,
    /// Domain name that this error applies to.
    #[serde(default)]
    pub domain_name: String,
    /// HTTP-equivalent status code for this item's failure (e.g. 403 for an unsupported extension).
    #[serde(default)]
    pub status_code: i64,
    /// Short error category or exception type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Human-readable reason this domain could not be resolved.
    #[serde(default)]
    pub message: String,
}

impl BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemError {
    pub fn builder() -> BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemErrorBuilder {
        <BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemErrorBuilder {
    status: Option<bool>,
    domain_name: Option<String>,
    status_code: Option<i64>,
    error: Option<String>,
    message: Option<String>,
}

impl BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemErrorBuilder {
    pub fn status(mut self, value: bool) -> Self {
        self.status = Some(value);
        self
    }

    pub fn domain_name(mut self, value: impl Into<String>) -> Self {
        self.domain_name = Some(value.into());
        self
    }

    pub fn status_code(mut self, value: i64) -> Self {
        self.status_code = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemErrorBuilder::status)
    /// - [`domain_name`](BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemErrorBuilder::domain_name)
    /// - [`status_code`](BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemErrorBuilder::status_code)
    /// - [`message`](BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemErrorBuilder::message)
    pub fn build(
        self,
    ) -> Result<BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemError, BuildError> {
        Ok(BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemError {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            domain_name: self
                .domain_name
                .ok_or_else(|| BuildError::missing_field("domain_name"))?,
            status_code: self
                .status_code
                .ok_or_else(|| BuildError::missing_field("status_code"))?,
            error: self.error,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
