pub use crate::prelude::*;

/// Threat intelligence details for the indicator of compromise (IOC).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DomainReputationResponseIntelligence {
    /// Type of the indicator of compromise (e.g. domain).
    #[serde(default)]
    pub ioc_type: String,
    /// Value of the indicator of compromise.
    #[serde(default)]
    pub ioc_value: String,
    /// Other IOCs related to this domain.
    #[serde(default)]
    pub related_iocs: Vec<DomainReputationResponseIntelligenceRelatedIocsItem>,
    /// Tags associated with this IOC from threat feeds.
    #[serde(default)]
    pub feed_tags: Vec<String>,
    /// STIX 2.1 pattern representation of the IOC, ready to wrap into an Indicator object.
    #[serde(default)]
    pub stix_pattern: String,
    /// Recommended action based on the assessment.
    pub recommended_action: DomainReputationResponseIntelligenceRecommendedAction,
    /// First time this IOC was observed (YYYY-MM-DDTHH:mm:ssZ). null when never observed on a feed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen: Option<String>,
    /// Last time this IOC was observed (YYYY-MM-DDTHH:mm:ssZ). null when never observed on a feed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
}

impl DomainReputationResponseIntelligence {
    pub fn builder() -> DomainReputationResponseIntelligenceBuilder {
        <DomainReputationResponseIntelligenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainReputationResponseIntelligenceBuilder {
    ioc_type: Option<String>,
    ioc_value: Option<String>,
    related_iocs: Option<Vec<DomainReputationResponseIntelligenceRelatedIocsItem>>,
    feed_tags: Option<Vec<String>>,
    stix_pattern: Option<String>,
    recommended_action: Option<DomainReputationResponseIntelligenceRecommendedAction>,
    first_seen: Option<String>,
    last_seen: Option<String>,
}

impl DomainReputationResponseIntelligenceBuilder {
    pub fn ioc_type(mut self, value: impl Into<String>) -> Self {
        self.ioc_type = Some(value.into());
        self
    }

    pub fn ioc_value(mut self, value: impl Into<String>) -> Self {
        self.ioc_value = Some(value.into());
        self
    }

    pub fn related_iocs(
        mut self,
        value: Vec<DomainReputationResponseIntelligenceRelatedIocsItem>,
    ) -> Self {
        self.related_iocs = Some(value);
        self
    }

    pub fn feed_tags(mut self, value: Vec<String>) -> Self {
        self.feed_tags = Some(value);
        self
    }

    pub fn stix_pattern(mut self, value: impl Into<String>) -> Self {
        self.stix_pattern = Some(value.into());
        self
    }

    pub fn recommended_action(
        mut self,
        value: DomainReputationResponseIntelligenceRecommendedAction,
    ) -> Self {
        self.recommended_action = Some(value);
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

    /// Consumes the builder and constructs a [`DomainReputationResponseIntelligence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ioc_type`](DomainReputationResponseIntelligenceBuilder::ioc_type)
    /// - [`ioc_value`](DomainReputationResponseIntelligenceBuilder::ioc_value)
    /// - [`related_iocs`](DomainReputationResponseIntelligenceBuilder::related_iocs)
    /// - [`feed_tags`](DomainReputationResponseIntelligenceBuilder::feed_tags)
    /// - [`stix_pattern`](DomainReputationResponseIntelligenceBuilder::stix_pattern)
    /// - [`recommended_action`](DomainReputationResponseIntelligenceBuilder::recommended_action)
    pub fn build(self) -> Result<DomainReputationResponseIntelligence, BuildError> {
        Ok(DomainReputationResponseIntelligence {
            ioc_type: self
                .ioc_type
                .ok_or_else(|| BuildError::missing_field("ioc_type"))?,
            ioc_value: self
                .ioc_value
                .ok_or_else(|| BuildError::missing_field("ioc_value"))?,
            related_iocs: self
                .related_iocs
                .ok_or_else(|| BuildError::missing_field("related_iocs"))?,
            feed_tags: self
                .feed_tags
                .ok_or_else(|| BuildError::missing_field("feed_tags"))?,
            stix_pattern: self
                .stix_pattern
                .ok_or_else(|| BuildError::missing_field("stix_pattern"))?,
            recommended_action: self
                .recommended_action
                .ok_or_else(|| BuildError::missing_field("recommended_action"))?,
            first_seen: self.first_seen,
            last_seen: self.last_seen,
        })
    }
}
