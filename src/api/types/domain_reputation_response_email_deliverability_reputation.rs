pub use crate::prelude::*;

/// Reputation and trust signals related to the domain's email sending history.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainReputationResponseEmailDeliverabilityReputation {
    /// Indicates whether the domain appears on known spam blacklists.
    #[serde(default)]
    pub spam_blacklisted: bool,
    /// Indicates whether the domain was registered recently.
    #[serde(default)]
    pub newly_registered: bool,
    /// Age of the domain in days since registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_age_days: Option<i64>,
}

impl DomainReputationResponseEmailDeliverabilityReputation {
    pub fn builder() -> DomainReputationResponseEmailDeliverabilityReputationBuilder {
        <DomainReputationResponseEmailDeliverabilityReputationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainReputationResponseEmailDeliverabilityReputationBuilder {
    spam_blacklisted: Option<bool>,
    newly_registered: Option<bool>,
    domain_age_days: Option<i64>,
}

impl DomainReputationResponseEmailDeliverabilityReputationBuilder {
    pub fn spam_blacklisted(mut self, value: bool) -> Self {
        self.spam_blacklisted = Some(value);
        self
    }

    pub fn newly_registered(mut self, value: bool) -> Self {
        self.newly_registered = Some(value);
        self
    }

    pub fn domain_age_days(mut self, value: i64) -> Self {
        self.domain_age_days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainReputationResponseEmailDeliverabilityReputation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`spam_blacklisted`](DomainReputationResponseEmailDeliverabilityReputationBuilder::spam_blacklisted)
    /// - [`newly_registered`](DomainReputationResponseEmailDeliverabilityReputationBuilder::newly_registered)
    pub fn build(
        self,
    ) -> Result<DomainReputationResponseEmailDeliverabilityReputation, BuildError> {
        Ok(DomainReputationResponseEmailDeliverabilityReputation {
            spam_blacklisted: self
                .spam_blacklisted
                .ok_or_else(|| BuildError::missing_field("spam_blacklisted"))?,
            newly_registered: self
                .newly_registered
                .ok_or_else(|| BuildError::missing_field("newly_registered"))?,
            domain_age_days: self.domain_age_days,
        })
    }
}
