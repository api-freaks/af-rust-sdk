pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainAvailabilityCheckResponse {
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "domainAvailability")]
    #[serde(default)]
    pub domain_availability: bool,
}

impl DomainAvailabilityCheckResponse {
    pub fn builder() -> DomainAvailabilityCheckResponseBuilder {
        <DomainAvailabilityCheckResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainAvailabilityCheckResponseBuilder {
    domain: Option<String>,
    domain_availability: Option<bool>,
}

impl DomainAvailabilityCheckResponseBuilder {
    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn domain_availability(mut self, value: bool) -> Self {
        self.domain_availability = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainAvailabilityCheckResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain`](DomainAvailabilityCheckResponseBuilder::domain)
    /// - [`domain_availability`](DomainAvailabilityCheckResponseBuilder::domain_availability)
    pub fn build(self) -> Result<DomainAvailabilityCheckResponse, BuildError> {
        Ok(DomainAvailabilityCheckResponse {
            domain: self
                .domain
                .ok_or_else(|| BuildError::missing_field("domain"))?,
            domain_availability: self
                .domain_availability
                .ok_or_else(|| BuildError::missing_field("domain_availability"))?,
        })
    }
}
