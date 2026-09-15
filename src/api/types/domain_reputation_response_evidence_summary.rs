pub use crate::prelude::*;

/// Summary of reasons behind the risk assessment.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainReputationResponseEvidenceSummary {
    /// List of reasons why the domain was flagged. Empty for a clean domain.
    #[serde(default)]
    pub why_flagged: Vec<String>,
}

impl DomainReputationResponseEvidenceSummary {
    pub fn builder() -> DomainReputationResponseEvidenceSummaryBuilder {
        <DomainReputationResponseEvidenceSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainReputationResponseEvidenceSummaryBuilder {
    why_flagged: Option<Vec<String>>,
}

impl DomainReputationResponseEvidenceSummaryBuilder {
    pub fn why_flagged(mut self, value: Vec<String>) -> Self {
        self.why_flagged = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainReputationResponseEvidenceSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`why_flagged`](DomainReputationResponseEvidenceSummaryBuilder::why_flagged)
    pub fn build(self) -> Result<DomainReputationResponseEvidenceSummary, BuildError> {
        Ok(DomainReputationResponseEvidenceSummary {
            why_flagged: self
                .why_flagged
                .ok_or_else(|| BuildError::missing_field("why_flagged"))?,
        })
    }
}
