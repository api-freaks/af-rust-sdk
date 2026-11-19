pub use crate::prelude::*;

/// Returned when `sug=false` — availability for the queried domain only, no suggestions.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
// required fields kept strict for untagged-union matching (fix_rust_untagged_union_required.py)
pub struct DomainAvailabilitySuggestionsResponseDomain {
    pub domain: String,
    #[serde(rename = "domainAvailability")]
    pub domain_availability: bool,
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
    /// This method will fail if any of the following fields are not set:
    /// - [`domain`](DomainAvailabilitySuggestionsResponseDomainBuilder::domain)
    /// - [`domain_availability`](DomainAvailabilitySuggestionsResponseDomainBuilder::domain_availability)
    pub fn build(self) -> Result<DomainAvailabilitySuggestionsResponseDomain, BuildError> {
        Ok(DomainAvailabilitySuggestionsResponseDomain {
            domain: self
                .domain
                .ok_or_else(|| BuildError::missing_field("domain"))?,
            domain_availability: self
                .domain_availability
                .ok_or_else(|| BuildError::missing_field("domain_availability"))?,
        })
    }
}
