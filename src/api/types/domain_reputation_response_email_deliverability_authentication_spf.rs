pub use crate::prelude::*;

/// Sender Policy Framework configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainReputationResponseEmailDeliverabilityAuthenticationSpf {
    /// Indicates whether an SPF record was found.
    #[serde(default)]
    pub present: bool,
    /// SPF enforcement policy qualifier found in the record (e.g. ~all, -all).
    #[serde(default)]
    pub policy: String,
    /// Raw SPF DNS TXT record string.
    #[serde(default)]
    pub record: String,
}

impl DomainReputationResponseEmailDeliverabilityAuthenticationSpf {
    pub fn builder() -> DomainReputationResponseEmailDeliverabilityAuthenticationSpfBuilder {
        <DomainReputationResponseEmailDeliverabilityAuthenticationSpfBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainReputationResponseEmailDeliverabilityAuthenticationSpfBuilder {
    present: Option<bool>,
    policy: Option<String>,
    record: Option<String>,
}

impl DomainReputationResponseEmailDeliverabilityAuthenticationSpfBuilder {
    pub fn present(mut self, value: bool) -> Self {
        self.present = Some(value);
        self
    }

    pub fn policy(mut self, value: impl Into<String>) -> Self {
        self.policy = Some(value.into());
        self
    }

    pub fn record(mut self, value: impl Into<String>) -> Self {
        self.record = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainReputationResponseEmailDeliverabilityAuthenticationSpf`].
    /// This method will fail if any of the following fields are not set:
    /// - [`present`](DomainReputationResponseEmailDeliverabilityAuthenticationSpfBuilder::present)
    /// - [`policy`](DomainReputationResponseEmailDeliverabilityAuthenticationSpfBuilder::policy)
    /// - [`record`](DomainReputationResponseEmailDeliverabilityAuthenticationSpfBuilder::record)
    pub fn build(
        self,
    ) -> Result<DomainReputationResponseEmailDeliverabilityAuthenticationSpf, BuildError> {
        Ok(
            DomainReputationResponseEmailDeliverabilityAuthenticationSpf {
                present: self
                    .present
                    .ok_or_else(|| BuildError::missing_field("present"))?,
                policy: self
                    .policy
                    .ok_or_else(|| BuildError::missing_field("policy"))?,
                record: self
                    .record
                    .ok_or_else(|| BuildError::missing_field("record"))?,
            },
        )
    }
}
