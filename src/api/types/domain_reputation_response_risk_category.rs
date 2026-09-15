pub use crate::prelude::*;

/// Overall risk assessment for the domain.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DomainReputationResponseRiskCategory {
    /// Final verdict of the risk assessment.
    pub verdict: DomainReputationResponseRiskCategoryVerdict,
    /// Confidence score for the verdict (0-1).
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub confidence: f64,
    /// Main threat type identified (e.g. phishing). null when no threat was identified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_threat: Option<String>,
    /// Severity level of the risk.
    pub severity: DomainReputationResponseRiskCategorySeverity,
    /// List of threat types associated with the domain.
    #[serde(default)]
    pub threat_types: Vec<String>,
    /// Threat intelligence sources that flagged the domain. Empty when nothing flagged it.
    #[serde(default)]
    pub sources: Vec<DomainReputationResponseRiskCategorySourcesItem>,
    /// Related pivots (nameserver, email, etc.) linked to known threats.
    #[serde(default)]
    pub pivot_matches: Vec<DomainReputationResponseRiskCategoryPivotMatchesItem>,
}

impl DomainReputationResponseRiskCategory {
    pub fn builder() -> DomainReputationResponseRiskCategoryBuilder {
        <DomainReputationResponseRiskCategoryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainReputationResponseRiskCategoryBuilder {
    verdict: Option<DomainReputationResponseRiskCategoryVerdict>,
    confidence: Option<f64>,
    primary_threat: Option<String>,
    severity: Option<DomainReputationResponseRiskCategorySeverity>,
    threat_types: Option<Vec<String>>,
    sources: Option<Vec<DomainReputationResponseRiskCategorySourcesItem>>,
    pivot_matches: Option<Vec<DomainReputationResponseRiskCategoryPivotMatchesItem>>,
}

impl DomainReputationResponseRiskCategoryBuilder {
    pub fn verdict(mut self, value: DomainReputationResponseRiskCategoryVerdict) -> Self {
        self.verdict = Some(value);
        self
    }

    pub fn confidence(mut self, value: f64) -> Self {
        self.confidence = Some(value);
        self
    }

    pub fn primary_threat(mut self, value: impl Into<String>) -> Self {
        self.primary_threat = Some(value.into());
        self
    }

    pub fn severity(mut self, value: DomainReputationResponseRiskCategorySeverity) -> Self {
        self.severity = Some(value);
        self
    }

    pub fn threat_types(mut self, value: Vec<String>) -> Self {
        self.threat_types = Some(value);
        self
    }

    pub fn sources(mut self, value: Vec<DomainReputationResponseRiskCategorySourcesItem>) -> Self {
        self.sources = Some(value);
        self
    }

    pub fn pivot_matches(
        mut self,
        value: Vec<DomainReputationResponseRiskCategoryPivotMatchesItem>,
    ) -> Self {
        self.pivot_matches = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainReputationResponseRiskCategory`].
    /// This method will fail if any of the following fields are not set:
    /// - [`verdict`](DomainReputationResponseRiskCategoryBuilder::verdict)
    /// - [`confidence`](DomainReputationResponseRiskCategoryBuilder::confidence)
    /// - [`severity`](DomainReputationResponseRiskCategoryBuilder::severity)
    /// - [`threat_types`](DomainReputationResponseRiskCategoryBuilder::threat_types)
    /// - [`sources`](DomainReputationResponseRiskCategoryBuilder::sources)
    /// - [`pivot_matches`](DomainReputationResponseRiskCategoryBuilder::pivot_matches)
    pub fn build(self) -> Result<DomainReputationResponseRiskCategory, BuildError> {
        Ok(DomainReputationResponseRiskCategory {
            verdict: self
                .verdict
                .ok_or_else(|| BuildError::missing_field("verdict"))?,
            confidence: self
                .confidence
                .ok_or_else(|| BuildError::missing_field("confidence"))?,
            primary_threat: self.primary_threat,
            severity: self
                .severity
                .ok_or_else(|| BuildError::missing_field("severity"))?,
            threat_types: self
                .threat_types
                .ok_or_else(|| BuildError::missing_field("threat_types"))?,
            sources: self
                .sources
                .ok_or_else(|| BuildError::missing_field("sources"))?,
            pivot_matches: self
                .pivot_matches
                .ok_or_else(|| BuildError::missing_field("pivot_matches"))?,
        })
    }
}
