pub use crate::prelude::*;

/// DomainKeys Identified Mail configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainReputationResponseEmailDeliverabilityAuthenticationDkim {
    /// Indicates whether a DKIM record was found for any probed selector.
    #[serde(default)]
    pub found: bool,
    /// List of DKIM selectors for which a record was found.
    #[serde(default)]
    pub selectors_found: Vec<String>,
    /// Email service providers inferred from the matched DKIM selectors.
    #[serde(default)]
    pub providers_detected: Vec<String>,
    /// Clarifying note about the limitations of DKIM selector probing.
    #[serde(default)]
    pub note: String,
}

impl DomainReputationResponseEmailDeliverabilityAuthenticationDkim {
    pub fn builder() -> DomainReputationResponseEmailDeliverabilityAuthenticationDkimBuilder {
        <DomainReputationResponseEmailDeliverabilityAuthenticationDkimBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainReputationResponseEmailDeliverabilityAuthenticationDkimBuilder {
    found: Option<bool>,
    selectors_found: Option<Vec<String>>,
    providers_detected: Option<Vec<String>>,
    note: Option<String>,
}

impl DomainReputationResponseEmailDeliverabilityAuthenticationDkimBuilder {
    pub fn found(mut self, value: bool) -> Self {
        self.found = Some(value);
        self
    }

    pub fn selectors_found(mut self, value: Vec<String>) -> Self {
        self.selectors_found = Some(value);
        self
    }

    pub fn providers_detected(mut self, value: Vec<String>) -> Self {
        self.providers_detected = Some(value);
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainReputationResponseEmailDeliverabilityAuthenticationDkim`].
    /// This method will fail if any of the following fields are not set:
    /// - [`found`](DomainReputationResponseEmailDeliverabilityAuthenticationDkimBuilder::found)
    /// - [`selectors_found`](DomainReputationResponseEmailDeliverabilityAuthenticationDkimBuilder::selectors_found)
    /// - [`providers_detected`](DomainReputationResponseEmailDeliverabilityAuthenticationDkimBuilder::providers_detected)
    /// - [`note`](DomainReputationResponseEmailDeliverabilityAuthenticationDkimBuilder::note)
    pub fn build(
        self,
    ) -> Result<DomainReputationResponseEmailDeliverabilityAuthenticationDkim, BuildError> {
        Ok(
            DomainReputationResponseEmailDeliverabilityAuthenticationDkim {
                found: self
                    .found
                    .ok_or_else(|| BuildError::missing_field("found"))?,
                selectors_found: self
                    .selectors_found
                    .ok_or_else(|| BuildError::missing_field("selectors_found"))?,
                providers_detected: self
                    .providers_detected
                    .ok_or_else(|| BuildError::missing_field("providers_detected"))?,
                note: self.note.ok_or_else(|| BuildError::missing_field("note"))?,
            },
        )
    }
}
