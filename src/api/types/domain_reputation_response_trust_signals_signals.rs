pub use crate::prelude::*;

/// Signals contributing to the trust score.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DomainReputationResponseTrustSignalsSignals {
    /// Signals that positively affect the trust score.
    #[serde(default)]
    pub positive: Vec<DomainReputationResponseTrustSignalsSignalsPositiveItem>,
    /// Signals that negatively affect the trust score.
    #[serde(default)]
    pub negative: Vec<DomainReputationResponseTrustSignalsSignalsNegativeItem>,
    /// Signals that are neutral to the trust score.
    #[serde(default)]
    pub neutral: Vec<DomainReputationResponseTrustSignalsSignalsNeutralItem>,
}

impl DomainReputationResponseTrustSignalsSignals {
    pub fn builder() -> DomainReputationResponseTrustSignalsSignalsBuilder {
        <DomainReputationResponseTrustSignalsSignalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainReputationResponseTrustSignalsSignalsBuilder {
    positive: Option<Vec<DomainReputationResponseTrustSignalsSignalsPositiveItem>>,
    negative: Option<Vec<DomainReputationResponseTrustSignalsSignalsNegativeItem>>,
    neutral: Option<Vec<DomainReputationResponseTrustSignalsSignalsNeutralItem>>,
}

impl DomainReputationResponseTrustSignalsSignalsBuilder {
    pub fn positive(
        mut self,
        value: Vec<DomainReputationResponseTrustSignalsSignalsPositiveItem>,
    ) -> Self {
        self.positive = Some(value);
        self
    }

    pub fn negative(
        mut self,
        value: Vec<DomainReputationResponseTrustSignalsSignalsNegativeItem>,
    ) -> Self {
        self.negative = Some(value);
        self
    }

    pub fn neutral(
        mut self,
        value: Vec<DomainReputationResponseTrustSignalsSignalsNeutralItem>,
    ) -> Self {
        self.neutral = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainReputationResponseTrustSignalsSignals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`positive`](DomainReputationResponseTrustSignalsSignalsBuilder::positive)
    /// - [`negative`](DomainReputationResponseTrustSignalsSignalsBuilder::negative)
    /// - [`neutral`](DomainReputationResponseTrustSignalsSignalsBuilder::neutral)
    pub fn build(self) -> Result<DomainReputationResponseTrustSignalsSignals, BuildError> {
        Ok(DomainReputationResponseTrustSignalsSignals {
            positive: self
                .positive
                .ok_or_else(|| BuildError::missing_field("positive"))?,
            negative: self
                .negative
                .ok_or_else(|| BuildError::missing_field("negative"))?,
            neutral: self
                .neutral
                .ok_or_else(|| BuildError::missing_field("neutral"))?,
        })
    }
}
