pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubdomainsLookupResponseSubdomainsItem {
    #[serde(default)]
    pub subdomain: String,
    #[serde(default)]
    pub first_seen: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
    /// The date from which the subdomain is considered inactive. Appears only if the subdomain is no longer active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_from: Option<String>,
}

impl SubdomainsLookupResponseSubdomainsItem {
    pub fn builder() -> SubdomainsLookupResponseSubdomainsItemBuilder {
        <SubdomainsLookupResponseSubdomainsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubdomainsLookupResponseSubdomainsItemBuilder {
    subdomain: Option<String>,
    first_seen: Option<String>,
    last_seen: Option<String>,
    inactive_from: Option<String>,
}

impl SubdomainsLookupResponseSubdomainsItemBuilder {
    pub fn subdomain(mut self, value: impl Into<String>) -> Self {
        self.subdomain = Some(value.into());
        self
    }

    pub fn first_seen(mut self, value: impl Into<String>) -> Self {
        self.first_seen = Some(value.into());
        self
    }

    pub fn last_seen(mut self, value: impl Into<String>) -> Self {
        self.last_seen = Some(value.into());
        self
    }

    pub fn inactive_from(mut self, value: impl Into<String>) -> Self {
        self.inactive_from = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubdomainsLookupResponseSubdomainsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`subdomain`](SubdomainsLookupResponseSubdomainsItemBuilder::subdomain)
    /// - [`first_seen`](SubdomainsLookupResponseSubdomainsItemBuilder::first_seen)
    pub fn build(self) -> Result<SubdomainsLookupResponseSubdomainsItem, BuildError> {
        Ok(SubdomainsLookupResponseSubdomainsItem {
            subdomain: self
                .subdomain
                .ok_or_else(|| BuildError::missing_field("subdomain"))?,
            first_seen: self
                .first_seen
                .ok_or_else(|| BuildError::missing_field("first_seen"))?,
            last_seen: self.last_seen,
            inactive_from: self.inactive_from,
        })
    }
}
