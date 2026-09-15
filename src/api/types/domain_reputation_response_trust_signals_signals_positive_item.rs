pub use crate::prelude::*;

/// A single trust signal contributing to the trust score.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DomainReputationResponseTrustSignalsSignalsPositiveItem {
    /// Signal code identifier (e.g. valid_ssl, dmarc_missing).
    #[serde(default)]
    pub code: String,
    /// Weight assigned to the signal.
    #[serde(default)]
    pub weight: i64,
    /// Polarity of the signal.
    pub polarity: DomainReputationResponseTrustSignalsSignalsPositiveItemPolarity,
    /// Category the signal belongs to (e.g. ssl_certificate).
    #[serde(default)]
    pub category: String,
    /// Evidence supporting the signal.
    #[serde(default)]
    pub evidence: String,
    /// Confidence score for the signal (0-1).
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub confidence: f64,
}

impl DomainReputationResponseTrustSignalsSignalsPositiveItem {
    pub fn builder() -> DomainReputationResponseTrustSignalsSignalsPositiveItemBuilder {
        <DomainReputationResponseTrustSignalsSignalsPositiveItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainReputationResponseTrustSignalsSignalsPositiveItemBuilder {
    code: Option<String>,
    weight: Option<i64>,
    polarity: Option<DomainReputationResponseTrustSignalsSignalsPositiveItemPolarity>,
    category: Option<String>,
    evidence: Option<String>,
    confidence: Option<f64>,
}

impl DomainReputationResponseTrustSignalsSignalsPositiveItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn weight(mut self, value: i64) -> Self {
        self.weight = Some(value);
        self
    }

    pub fn polarity(
        mut self,
        value: DomainReputationResponseTrustSignalsSignalsPositiveItemPolarity,
    ) -> Self {
        self.polarity = Some(value);
        self
    }

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn evidence(mut self, value: impl Into<String>) -> Self {
        self.evidence = Some(value.into());
        self
    }

    pub fn confidence(mut self, value: f64) -> Self {
        self.confidence = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainReputationResponseTrustSignalsSignalsPositiveItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](DomainReputationResponseTrustSignalsSignalsPositiveItemBuilder::code)
    /// - [`weight`](DomainReputationResponseTrustSignalsSignalsPositiveItemBuilder::weight)
    /// - [`polarity`](DomainReputationResponseTrustSignalsSignalsPositiveItemBuilder::polarity)
    /// - [`category`](DomainReputationResponseTrustSignalsSignalsPositiveItemBuilder::category)
    /// - [`evidence`](DomainReputationResponseTrustSignalsSignalsPositiveItemBuilder::evidence)
    /// - [`confidence`](DomainReputationResponseTrustSignalsSignalsPositiveItemBuilder::confidence)
    pub fn build(
        self,
    ) -> Result<DomainReputationResponseTrustSignalsSignalsPositiveItem, BuildError> {
        Ok(DomainReputationResponseTrustSignalsSignalsPositiveItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            weight: self
                .weight
                .ok_or_else(|| BuildError::missing_field("weight"))?,
            polarity: self
                .polarity
                .ok_or_else(|| BuildError::missing_field("polarity"))?,
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            evidence: self
                .evidence
                .ok_or_else(|| BuildError::missing_field("evidence"))?,
            confidence: self
                .confidence
                .ok_or_else(|| BuildError::missing_field("confidence"))?,
        })
    }
}
