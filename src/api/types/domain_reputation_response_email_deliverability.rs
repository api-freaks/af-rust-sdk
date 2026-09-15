pub use crate::prelude::*;

/// Assessment of the domain's ability to send and receive email reliably.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainReputationResponseEmailDeliverability {
    /// Overall email deliverability score out of 100.
    #[serde(default)]
    pub score: i64,
    /// Letter / word grade summarizing the deliverability score.
    #[serde(default)]
    pub grade: String,
    /// Indicates whether the domain is configured to receive email.
    #[serde(default)]
    pub can_receive_email: bool,
    /// Email authentication mechanisms configured for the domain.
    #[serde(default)]
    pub authentication: DomainReputationResponseEmailDeliverabilityAuthentication,
    /// Mail server infrastructure backing the domain.
    #[serde(default)]
    pub infrastructure: DomainReputationResponseEmailDeliverabilityInfrastructure,
    /// Reputation and trust signals related to the domain's email sending history.
    #[serde(default)]
    pub reputation: DomainReputationResponseEmailDeliverabilityReputation,
    /// List of detected email deliverability issues or misconfigurations.
    #[serde(default)]
    pub issues: Vec<DomainReputationResponseEmailDeliverabilityIssuesItem>,
}

impl DomainReputationResponseEmailDeliverability {
    pub fn builder() -> DomainReputationResponseEmailDeliverabilityBuilder {
        <DomainReputationResponseEmailDeliverabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainReputationResponseEmailDeliverabilityBuilder {
    score: Option<i64>,
    grade: Option<String>,
    can_receive_email: Option<bool>,
    authentication: Option<DomainReputationResponseEmailDeliverabilityAuthentication>,
    infrastructure: Option<DomainReputationResponseEmailDeliverabilityInfrastructure>,
    reputation: Option<DomainReputationResponseEmailDeliverabilityReputation>,
    issues: Option<Vec<DomainReputationResponseEmailDeliverabilityIssuesItem>>,
}

impl DomainReputationResponseEmailDeliverabilityBuilder {
    pub fn score(mut self, value: i64) -> Self {
        self.score = Some(value);
        self
    }

    pub fn grade(mut self, value: impl Into<String>) -> Self {
        self.grade = Some(value.into());
        self
    }

    pub fn can_receive_email(mut self, value: bool) -> Self {
        self.can_receive_email = Some(value);
        self
    }

    pub fn authentication(
        mut self,
        value: DomainReputationResponseEmailDeliverabilityAuthentication,
    ) -> Self {
        self.authentication = Some(value);
        self
    }

    pub fn infrastructure(
        mut self,
        value: DomainReputationResponseEmailDeliverabilityInfrastructure,
    ) -> Self {
        self.infrastructure = Some(value);
        self
    }

    pub fn reputation(
        mut self,
        value: DomainReputationResponseEmailDeliverabilityReputation,
    ) -> Self {
        self.reputation = Some(value);
        self
    }

    pub fn issues(
        mut self,
        value: Vec<DomainReputationResponseEmailDeliverabilityIssuesItem>,
    ) -> Self {
        self.issues = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainReputationResponseEmailDeliverability`].
    /// This method will fail if any of the following fields are not set:
    /// - [`score`](DomainReputationResponseEmailDeliverabilityBuilder::score)
    /// - [`grade`](DomainReputationResponseEmailDeliverabilityBuilder::grade)
    /// - [`can_receive_email`](DomainReputationResponseEmailDeliverabilityBuilder::can_receive_email)
    /// - [`authentication`](DomainReputationResponseEmailDeliverabilityBuilder::authentication)
    /// - [`infrastructure`](DomainReputationResponseEmailDeliverabilityBuilder::infrastructure)
    /// - [`reputation`](DomainReputationResponseEmailDeliverabilityBuilder::reputation)
    /// - [`issues`](DomainReputationResponseEmailDeliverabilityBuilder::issues)
    pub fn build(self) -> Result<DomainReputationResponseEmailDeliverability, BuildError> {
        Ok(DomainReputationResponseEmailDeliverability {
            score: self
                .score
                .ok_or_else(|| BuildError::missing_field("score"))?,
            grade: self
                .grade
                .ok_or_else(|| BuildError::missing_field("grade"))?,
            can_receive_email: self
                .can_receive_email
                .ok_or_else(|| BuildError::missing_field("can_receive_email"))?,
            authentication: self
                .authentication
                .ok_or_else(|| BuildError::missing_field("authentication"))?,
            infrastructure: self
                .infrastructure
                .ok_or_else(|| BuildError::missing_field("infrastructure"))?,
            reputation: self
                .reputation
                .ok_or_else(|| BuildError::missing_field("reputation"))?,
            issues: self
                .issues
                .ok_or_else(|| BuildError::missing_field("issues"))?,
        })
    }
}
