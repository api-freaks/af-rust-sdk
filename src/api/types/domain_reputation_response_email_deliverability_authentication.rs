pub use crate::prelude::*;

/// Email authentication mechanisms configured for the domain.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainReputationResponseEmailDeliverabilityAuthentication {
    /// Sender Policy Framework configuration.
    #[serde(default)]
    pub spf: DomainReputationResponseEmailDeliverabilityAuthenticationSpf,
    /// DomainKeys Identified Mail configuration.
    #[serde(default)]
    pub dkim: DomainReputationResponseEmailDeliverabilityAuthenticationDkim,
    /// Domain-based Message Authentication, Reporting and Conformance configuration.
    #[serde(default)]
    pub dmarc: DomainReputationResponseEmailDeliverabilityAuthenticationDmarc,
}

impl DomainReputationResponseEmailDeliverabilityAuthentication {
    pub fn builder() -> DomainReputationResponseEmailDeliverabilityAuthenticationBuilder {
        <DomainReputationResponseEmailDeliverabilityAuthenticationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainReputationResponseEmailDeliverabilityAuthenticationBuilder {
    spf: Option<DomainReputationResponseEmailDeliverabilityAuthenticationSpf>,
    dkim: Option<DomainReputationResponseEmailDeliverabilityAuthenticationDkim>,
    dmarc: Option<DomainReputationResponseEmailDeliverabilityAuthenticationDmarc>,
}

impl DomainReputationResponseEmailDeliverabilityAuthenticationBuilder {
    pub fn spf(
        mut self,
        value: DomainReputationResponseEmailDeliverabilityAuthenticationSpf,
    ) -> Self {
        self.spf = Some(value);
        self
    }

    pub fn dkim(
        mut self,
        value: DomainReputationResponseEmailDeliverabilityAuthenticationDkim,
    ) -> Self {
        self.dkim = Some(value);
        self
    }

    pub fn dmarc(
        mut self,
        value: DomainReputationResponseEmailDeliverabilityAuthenticationDmarc,
    ) -> Self {
        self.dmarc = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainReputationResponseEmailDeliverabilityAuthentication`].
    /// This method will fail if any of the following fields are not set:
    /// - [`spf`](DomainReputationResponseEmailDeliverabilityAuthenticationBuilder::spf)
    /// - [`dkim`](DomainReputationResponseEmailDeliverabilityAuthenticationBuilder::dkim)
    /// - [`dmarc`](DomainReputationResponseEmailDeliverabilityAuthenticationBuilder::dmarc)
    pub fn build(
        self,
    ) -> Result<DomainReputationResponseEmailDeliverabilityAuthentication, BuildError> {
        Ok(DomainReputationResponseEmailDeliverabilityAuthentication {
            spf: self.spf.ok_or_else(|| BuildError::missing_field("spf"))?,
            dkim: self.dkim.ok_or_else(|| BuildError::missing_field("dkim"))?,
            dmarc: self
                .dmarc
                .ok_or_else(|| BuildError::missing_field("dmarc"))?,
        })
    }
}
