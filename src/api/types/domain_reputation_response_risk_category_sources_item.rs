pub use crate::prelude::*;

/// A threat intelligence source that flagged the domain.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DomainReputationResponseRiskCategorySourcesItem {
    /// Name of the threat intelligence source (e.g. Spamhaus).
    #[serde(default)]
    pub source: String,
    /// Indicator matched by this source.
    #[serde(default)]
    pub indicator: String,
    /// Threat type reported by this source.
    #[serde(default)]
    pub threat_type: String,
    /// Confidence score from this source (0-1).
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub confidence: f64,
    /// First time this indicator was seen by the source (YYYY-MM-DDTHH:mm:ssZ).
    #[serde(default)]
    pub first_seen: String,
    /// Last time this indicator was seen by the source (YYYY-MM-DDTHH:mm:ssZ).
    #[serde(default)]
    pub last_seen: String,
}

impl DomainReputationResponseRiskCategorySourcesItem {
    pub fn builder() -> DomainReputationResponseRiskCategorySourcesItemBuilder {
        <DomainReputationResponseRiskCategorySourcesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainReputationResponseRiskCategorySourcesItemBuilder {
    source: Option<String>,
    indicator: Option<String>,
    threat_type: Option<String>,
    confidence: Option<f64>,
    first_seen: Option<String>,
    last_seen: Option<String>,
}

impl DomainReputationResponseRiskCategorySourcesItemBuilder {
    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn indicator(mut self, value: impl Into<String>) -> Self {
        self.indicator = Some(value.into());
        self
    }

    pub fn threat_type(mut self, value: impl Into<String>) -> Self {
        self.threat_type = Some(value.into());
        self
    }

    pub fn confidence(mut self, value: f64) -> Self {
        self.confidence = Some(value);
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

    /// Consumes the builder and constructs a [`DomainReputationResponseRiskCategorySourcesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source`](DomainReputationResponseRiskCategorySourcesItemBuilder::source)
    /// - [`indicator`](DomainReputationResponseRiskCategorySourcesItemBuilder::indicator)
    /// - [`threat_type`](DomainReputationResponseRiskCategorySourcesItemBuilder::threat_type)
    /// - [`confidence`](DomainReputationResponseRiskCategorySourcesItemBuilder::confidence)
    /// - [`first_seen`](DomainReputationResponseRiskCategorySourcesItemBuilder::first_seen)
    /// - [`last_seen`](DomainReputationResponseRiskCategorySourcesItemBuilder::last_seen)
    pub fn build(self) -> Result<DomainReputationResponseRiskCategorySourcesItem, BuildError> {
        Ok(DomainReputationResponseRiskCategorySourcesItem {
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            indicator: self
                .indicator
                .ok_or_else(|| BuildError::missing_field("indicator"))?,
            threat_type: self
                .threat_type
                .ok_or_else(|| BuildError::missing_field("threat_type"))?,
            confidence: self
                .confidence
                .ok_or_else(|| BuildError::missing_field("confidence"))?,
            first_seen: self
                .first_seen
                .ok_or_else(|| BuildError::missing_field("first_seen"))?,
            last_seen: self
                .last_seen
                .ok_or_else(|| BuildError::missing_field("last_seen"))?,
        })
    }
}
