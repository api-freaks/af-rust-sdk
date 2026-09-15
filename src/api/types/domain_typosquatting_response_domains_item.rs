pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainTyposquattingResponseDomainsItem {
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
    /// Domain creation date (YYYY-MM-DD). May be absent for older or less-actively-tracked entries.
    #[serde(rename = "createDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// Domain expiration date (YYYY-MM-DD). May be absent for older or less-actively-tracked entries.
    #[serde(rename = "expiryDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    /// Last time the domain was observed (YYYY-MM-DD).
    #[serde(rename = "lastSeen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
    /// Indicates whether the domain has dropped out of the registry and become available to register again.
    #[serde(rename = "isDropped")]
    #[serde(default)]
    pub is_dropped: bool,
}

impl DomainTyposquattingResponseDomainsItem {
    pub fn builder() -> DomainTyposquattingResponseDomainsItemBuilder {
        <DomainTyposquattingResponseDomainsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainTyposquattingResponseDomainsItemBuilder {
    domain_name: Option<String>,
    create_date: Option<String>,
    expiry_date: Option<String>,
    last_seen: Option<String>,
    is_dropped: Option<bool>,
}

impl DomainTyposquattingResponseDomainsItemBuilder {
    pub fn domain_name(mut self, value: impl Into<String>) -> Self {
        self.domain_name = Some(value.into());
        self
    }

    pub fn create_date(mut self, value: impl Into<String>) -> Self {
        self.create_date = Some(value.into());
        self
    }

    pub fn expiry_date(mut self, value: impl Into<String>) -> Self {
        self.expiry_date = Some(value.into());
        self
    }

    pub fn last_seen(mut self, value: impl Into<String>) -> Self {
        self.last_seen = Some(value.into());
        self
    }

    pub fn is_dropped(mut self, value: bool) -> Self {
        self.is_dropped = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainTyposquattingResponseDomainsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain_name`](DomainTyposquattingResponseDomainsItemBuilder::domain_name)
    /// - [`is_dropped`](DomainTyposquattingResponseDomainsItemBuilder::is_dropped)
    pub fn build(self) -> Result<DomainTyposquattingResponseDomainsItem, BuildError> {
        Ok(DomainTyposquattingResponseDomainsItem {
            domain_name: self
                .domain_name
                .ok_or_else(|| BuildError::missing_field("domain_name"))?,
            create_date: self.create_date,
            expiry_date: self.expiry_date,
            last_seen: self.last_seen,
            is_dropped: self
                .is_dropped
                .ok_or_else(|| BuildError::missing_field("is_dropped"))?,
        })
    }
}
