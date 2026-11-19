pub use crate::prelude::*;

/// Backlink profile of the domain.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DomainReputationResponseBacklinks {
    /// true if backlink data exists for the domain. When false, the other fields are null.
    #[serde(default)]
    pub found: bool,
    /// Domain the backlink data was matched to (for example `google.com`). null when no data was found.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_domain: Option<String>,
    /// Total number of backlinks pointing to the domain (for example 174614443). null when no data was found.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backlink_count: Option<i64>,
    /// Number of distinct domains linking to the domain (for example 4336653). null when no data was found.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referring_domains: Option<i64>,
    /// Average number of backlinks per referring domain (backlink_count / referring_domains, for example 40.26). null when no data was found.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer_link_density: Option<f64>,
}

impl DomainReputationResponseBacklinks {
    pub fn builder() -> DomainReputationResponseBacklinksBuilder {
        <DomainReputationResponseBacklinksBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainReputationResponseBacklinksBuilder {
    found: Option<bool>,
    matched_domain: Option<String>,
    backlink_count: Option<i64>,
    referring_domains: Option<i64>,
    referrer_link_density: Option<f64>,
}

impl DomainReputationResponseBacklinksBuilder {
    pub fn found(mut self, value: bool) -> Self {
        self.found = Some(value);
        self
    }

    pub fn matched_domain(mut self, value: impl Into<String>) -> Self {
        self.matched_domain = Some(value.into());
        self
    }

    pub fn backlink_count(mut self, value: i64) -> Self {
        self.backlink_count = Some(value);
        self
    }

    pub fn referring_domains(mut self, value: i64) -> Self {
        self.referring_domains = Some(value);
        self
    }

    pub fn referrer_link_density(mut self, value: f64) -> Self {
        self.referrer_link_density = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainReputationResponseBacklinks`].
    /// This method will fail if any of the following fields are not set:
    /// - [`found`](DomainReputationResponseBacklinksBuilder::found)
    pub fn build(self) -> Result<DomainReputationResponseBacklinks, BuildError> {
        Ok(DomainReputationResponseBacklinks {
            found: self
                .found
                .ok_or_else(|| BuildError::missing_field("found"))?,
            matched_domain: self.matched_domain,
            backlink_count: self.backlink_count,
            referring_domains: self.referring_domains,
            referrer_link_density: self.referrer_link_density,
        })
    }
}
