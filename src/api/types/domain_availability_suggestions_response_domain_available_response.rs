pub use crate::prelude::*;

/// Returned when `sug` is omitted or `true` — the queried domain plus suggested alternatives.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
// required fields kept strict for untagged-union matching (fix_rust_untagged_union_required.py)
pub struct DomainAvailabilitySuggestionsResponseDomainAvailableResponse {
    pub domain_available_response: Vec<
        DomainAvailabilitySuggestionsResponseDomainAvailableResponseDomainAvailableResponseItem,
    >,
}

impl DomainAvailabilitySuggestionsResponseDomainAvailableResponse {
    pub fn builder() -> DomainAvailabilitySuggestionsResponseDomainAvailableResponseBuilder {
        <DomainAvailabilitySuggestionsResponseDomainAvailableResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainAvailabilitySuggestionsResponseDomainAvailableResponseBuilder {
    domain_available_response: Option<
        Vec<
            DomainAvailabilitySuggestionsResponseDomainAvailableResponseDomainAvailableResponseItem,
        >,
    >,
}

impl DomainAvailabilitySuggestionsResponseDomainAvailableResponseBuilder {
    pub fn domain_available_response(
        mut self,
        value: Vec<
            DomainAvailabilitySuggestionsResponseDomainAvailableResponseDomainAvailableResponseItem,
        >,
    ) -> Self {
        self.domain_available_response = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainAvailabilitySuggestionsResponseDomainAvailableResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain_available_response`](DomainAvailabilitySuggestionsResponseDomainAvailableResponseBuilder::domain_available_response)
    pub fn build(
        self,
    ) -> Result<DomainAvailabilitySuggestionsResponseDomainAvailableResponse, BuildError> {
        Ok(
            DomainAvailabilitySuggestionsResponseDomainAvailableResponse {
                domain_available_response: self
                    .domain_available_response
                    .ok_or_else(|| BuildError::missing_field("domain_available_response"))?,
            },
        )
    }
}
