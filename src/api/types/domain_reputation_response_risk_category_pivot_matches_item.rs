pub use crate::prelude::*;

/// A related pivot linked to known threats.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DomainReputationResponseRiskCategoryPivotMatchesItem {
    /// Pivot value (e.g. a nameserver or email address).
    #[serde(default)]
    pub pivot: String,
    /// Type of pivot.
    #[serde(default)]
    pub pivot_type: String,
    /// Total number of threats related to this pivot.
    #[serde(default)]
    pub total_related_threats: i64,
    /// Confidence score for the pivot match (0-1).
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub confidence: f64,
}

impl DomainReputationResponseRiskCategoryPivotMatchesItem {
    pub fn builder() -> DomainReputationResponseRiskCategoryPivotMatchesItemBuilder {
        <DomainReputationResponseRiskCategoryPivotMatchesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainReputationResponseRiskCategoryPivotMatchesItemBuilder {
    pivot: Option<String>,
    pivot_type: Option<String>,
    total_related_threats: Option<i64>,
    confidence: Option<f64>,
}

impl DomainReputationResponseRiskCategoryPivotMatchesItemBuilder {
    pub fn pivot(mut self, value: impl Into<String>) -> Self {
        self.pivot = Some(value.into());
        self
    }

    pub fn pivot_type(mut self, value: impl Into<String>) -> Self {
        self.pivot_type = Some(value.into());
        self
    }

    pub fn total_related_threats(mut self, value: i64) -> Self {
        self.total_related_threats = Some(value);
        self
    }

    pub fn confidence(mut self, value: f64) -> Self {
        self.confidence = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainReputationResponseRiskCategoryPivotMatchesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pivot`](DomainReputationResponseRiskCategoryPivotMatchesItemBuilder::pivot)
    /// - [`pivot_type`](DomainReputationResponseRiskCategoryPivotMatchesItemBuilder::pivot_type)
    /// - [`total_related_threats`](DomainReputationResponseRiskCategoryPivotMatchesItemBuilder::total_related_threats)
    /// - [`confidence`](DomainReputationResponseRiskCategoryPivotMatchesItemBuilder::confidence)
    pub fn build(self) -> Result<DomainReputationResponseRiskCategoryPivotMatchesItem, BuildError> {
        Ok(DomainReputationResponseRiskCategoryPivotMatchesItem {
            pivot: self
                .pivot
                .ok_or_else(|| BuildError::missing_field("pivot"))?,
            pivot_type: self
                .pivot_type
                .ok_or_else(|| BuildError::missing_field("pivot_type"))?,
            total_related_threats: self
                .total_related_threats
                .ok_or_else(|| BuildError::missing_field("total_related_threats"))?,
            confidence: self
                .confidence
                .ok_or_else(|| BuildError::missing_field("confidence"))?,
        })
    }
}
