pub use crate::prelude::*;

/// Returned when `sug=false` — availability for the queried domain only, no suggestions.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainAvailabilitySuggestionsResponseDomain {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "domainAvailability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_availability: Option<bool>,
}

impl DomainAvailabilitySuggestionsResponseDomain {
    pub fn builder() -> DomainAvailabilitySuggestionsResponseDomainBuilder {
        <DomainAvailabilitySuggestionsResponseDomainBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainAvailabilitySuggestionsResponseDomainBuilder {
    domain: Option<String>,
    domain_availability: Option<bool>,
}

impl DomainAvailabilitySuggestionsResponseDomainBuilder {
    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn domain_availability(mut self, value: bool) -> Self {
        self.domain_availability = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainAvailabilitySuggestionsResponseDomain`].
    pub fn build(self) -> Result<DomainAvailabilitySuggestionsResponseDomain, BuildError> {
        Ok(DomainAvailabilitySuggestionsResponseDomain {
            domain: self.domain,
            domain_availability: self.domain_availability,
        })
    }
}
