pub use crate::prelude::*;

/// Current registration status of the domain.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainReputationResponseRegistration {
    /// Registration status of the domain, for example `registered` or `not_registered`.
    #[serde(default)]
    pub status: String,
    /// Number of earlier registrations found for the domain. Returned when the domain is not currently registered (for example `0`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub past_registrations: Option<i64>,
}

impl DomainReputationResponseRegistration {
    pub fn builder() -> DomainReputationResponseRegistrationBuilder {
        <DomainReputationResponseRegistrationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainReputationResponseRegistrationBuilder {
    status: Option<String>,
    past_registrations: Option<i64>,
}

impl DomainReputationResponseRegistrationBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn past_registrations(mut self, value: i64) -> Self {
        self.past_registrations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainReputationResponseRegistration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](DomainReputationResponseRegistrationBuilder::status)
    pub fn build(self) -> Result<DomainReputationResponseRegistration, BuildError> {
        Ok(DomainReputationResponseRegistration {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            past_registrations: self.past_registrations,
        })
    }
}
