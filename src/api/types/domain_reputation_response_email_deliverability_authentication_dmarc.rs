pub use crate::prelude::*;

/// Domain-based Message Authentication, Reporting and Conformance configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainReputationResponseEmailDeliverabilityAuthenticationDmarc {
    /// Indicates whether a DMARC record was found.
    #[serde(default)]
    pub present: bool,
    /// DMARC enforcement policy applied to failing messages (e.g. none, quarantine, reject).
    #[serde(default)]
    pub policy: String,
    /// Indicates whether DMARC aggregate / forensic reporting addresses are configured.
    #[serde(default)]
    pub reporting_configured: bool,
}

impl DomainReputationResponseEmailDeliverabilityAuthenticationDmarc {
    pub fn builder() -> DomainReputationResponseEmailDeliverabilityAuthenticationDmarcBuilder {
        <DomainReputationResponseEmailDeliverabilityAuthenticationDmarcBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainReputationResponseEmailDeliverabilityAuthenticationDmarcBuilder {
    present: Option<bool>,
    policy: Option<String>,
    reporting_configured: Option<bool>,
}

impl DomainReputationResponseEmailDeliverabilityAuthenticationDmarcBuilder {
    pub fn present(mut self, value: bool) -> Self {
        self.present = Some(value);
        self
    }

    pub fn policy(mut self, value: impl Into<String>) -> Self {
        self.policy = Some(value.into());
        self
    }

    pub fn reporting_configured(mut self, value: bool) -> Self {
        self.reporting_configured = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainReputationResponseEmailDeliverabilityAuthenticationDmarc`].
    /// This method will fail if any of the following fields are not set:
    /// - [`present`](DomainReputationResponseEmailDeliverabilityAuthenticationDmarcBuilder::present)
    /// - [`policy`](DomainReputationResponseEmailDeliverabilityAuthenticationDmarcBuilder::policy)
    /// - [`reporting_configured`](DomainReputationResponseEmailDeliverabilityAuthenticationDmarcBuilder::reporting_configured)
    pub fn build(
        self,
    ) -> Result<DomainReputationResponseEmailDeliverabilityAuthenticationDmarc, BuildError> {
        Ok(
            DomainReputationResponseEmailDeliverabilityAuthenticationDmarc {
                present: self
                    .present
                    .ok_or_else(|| BuildError::missing_field("present"))?,
                policy: self
                    .policy
                    .ok_or_else(|| BuildError::missing_field("policy"))?,
                reporting_configured: self
                    .reporting_configured
                    .ok_or_else(|| BuildError::missing_field("reporting_configured"))?,
            },
        )
    }
}
