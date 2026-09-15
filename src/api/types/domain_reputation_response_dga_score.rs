pub use crate::prelude::*;

/// Domain Generation Algorithm (DGA) detection results.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DomainReputationResponseDgaScore {
    /// DGA likelihood score (0-1).
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub score: f64,
    /// Indicates whether the domain is likely DGA-generated.
    #[serde(default)]
    pub is_dga: bool,
    /// Model used to compute the DGA score.
    #[serde(default)]
    pub model: String,
    /// Underlying lexical / statistical features used in DGA detection.
    #[serde(default)]
    pub features: DomainReputationResponseDgaScoreFeatures,
    /// Human-readable interpretation of the DGA score.
    #[serde(default)]
    pub interpretation: String,
}

impl DomainReputationResponseDgaScore {
    pub fn builder() -> DomainReputationResponseDgaScoreBuilder {
        <DomainReputationResponseDgaScoreBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainReputationResponseDgaScoreBuilder {
    score: Option<f64>,
    is_dga: Option<bool>,
    model: Option<String>,
    features: Option<DomainReputationResponseDgaScoreFeatures>,
    interpretation: Option<String>,
}

impl DomainReputationResponseDgaScoreBuilder {
    pub fn score(mut self, value: f64) -> Self {
        self.score = Some(value);
        self
    }

    pub fn is_dga(mut self, value: bool) -> Self {
        self.is_dga = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn features(mut self, value: DomainReputationResponseDgaScoreFeatures) -> Self {
        self.features = Some(value);
        self
    }

    pub fn interpretation(mut self, value: impl Into<String>) -> Self {
        self.interpretation = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainReputationResponseDgaScore`].
    /// This method will fail if any of the following fields are not set:
    /// - [`score`](DomainReputationResponseDgaScoreBuilder::score)
    /// - [`is_dga`](DomainReputationResponseDgaScoreBuilder::is_dga)
    /// - [`model`](DomainReputationResponseDgaScoreBuilder::model)
    /// - [`features`](DomainReputationResponseDgaScoreBuilder::features)
    /// - [`interpretation`](DomainReputationResponseDgaScoreBuilder::interpretation)
    pub fn build(self) -> Result<DomainReputationResponseDgaScore, BuildError> {
        Ok(DomainReputationResponseDgaScore {
            score: self
                .score
                .ok_or_else(|| BuildError::missing_field("score"))?,
            is_dga: self
                .is_dga
                .ok_or_else(|| BuildError::missing_field("is_dga"))?,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            features: self
                .features
                .ok_or_else(|| BuildError::missing_field("features"))?,
            interpretation: self
                .interpretation
                .ok_or_else(|| BuildError::missing_field("interpretation"))?,
        })
    }
}
