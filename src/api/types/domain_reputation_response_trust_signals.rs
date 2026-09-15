pub use crate::prelude::*;

/// Trust scoring and supporting signals for the domain.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DomainReputationResponseTrustSignals {
    /// Overall trust score (0-100).
    #[serde(default)]
    pub trust_score: i64,
    /// Trust score band / category (e.g. low, medium, high).
    #[serde(default)]
    pub trust_band: String,
    /// Signals contributing to the trust score.
    #[serde(default)]
    pub signals: DomainReputationResponseTrustSignalsSignals,
    /// Individual trust / risk indicators for the domain.
    #[serde(default)]
    pub indicators: DomainReputationResponseTrustSignalsIndicators,
}

impl DomainReputationResponseTrustSignals {
    pub fn builder() -> DomainReputationResponseTrustSignalsBuilder {
        <DomainReputationResponseTrustSignalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainReputationResponseTrustSignalsBuilder {
    trust_score: Option<i64>,
    trust_band: Option<String>,
    signals: Option<DomainReputationResponseTrustSignalsSignals>,
    indicators: Option<DomainReputationResponseTrustSignalsIndicators>,
}

impl DomainReputationResponseTrustSignalsBuilder {
    pub fn trust_score(mut self, value: i64) -> Self {
        self.trust_score = Some(value);
        self
    }

    pub fn trust_band(mut self, value: impl Into<String>) -> Self {
        self.trust_band = Some(value.into());
        self
    }

    pub fn signals(mut self, value: DomainReputationResponseTrustSignalsSignals) -> Self {
        self.signals = Some(value);
        self
    }

    pub fn indicators(mut self, value: DomainReputationResponseTrustSignalsIndicators) -> Self {
        self.indicators = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainReputationResponseTrustSignals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`trust_score`](DomainReputationResponseTrustSignalsBuilder::trust_score)
    /// - [`trust_band`](DomainReputationResponseTrustSignalsBuilder::trust_band)
    /// - [`signals`](DomainReputationResponseTrustSignalsBuilder::signals)
    /// - [`indicators`](DomainReputationResponseTrustSignalsBuilder::indicators)
    pub fn build(self) -> Result<DomainReputationResponseTrustSignals, BuildError> {
        Ok(DomainReputationResponseTrustSignals {
            trust_score: self
                .trust_score
                .ok_or_else(|| BuildError::missing_field("trust_score"))?,
            trust_band: self
                .trust_band
                .ok_or_else(|| BuildError::missing_field("trust_band"))?,
            signals: self
                .signals
                .ok_or_else(|| BuildError::missing_field("signals"))?,
            indicators: self
                .indicators
                .ok_or_else(|| BuildError::missing_field("indicators"))?,
        })
    }
}
