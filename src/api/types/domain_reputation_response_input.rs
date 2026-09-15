pub use crate::prelude::*;

/// Input object containing the analyzed domain.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainReputationResponseInput {
    /// Domain name being analyzed.
    #[serde(default)]
    pub domain: String,
}

impl DomainReputationResponseInput {
    pub fn builder() -> DomainReputationResponseInputBuilder {
        <DomainReputationResponseInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainReputationResponseInputBuilder {
    domain: Option<String>,
}

impl DomainReputationResponseInputBuilder {
    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainReputationResponseInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain`](DomainReputationResponseInputBuilder::domain)
    pub fn build(self) -> Result<DomainReputationResponseInput, BuildError> {
        Ok(DomainReputationResponseInput {
            domain: self
                .domain
                .ok_or_else(|| BuildError::missing_field("domain"))?,
        })
    }
}
