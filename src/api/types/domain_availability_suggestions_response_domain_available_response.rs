pub use crate::prelude::*;

/// Returned when `sug` is omitted or `true` — the queried domain plus suggested alternatives.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct DomainAvailabilitySuggestionsResponseDomainAvailableResponse {
    pub domain_available_response: Option<
        Vec<
            DomainAvailabilitySuggestionsResponseDomainAvailableResponseDomainAvailableResponseItem,
        >,
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
    pub fn build(
        self,
    ) -> Result<DomainAvailabilitySuggestionsResponseDomainAvailableResponse, BuildError> {
        Ok(
            DomainAvailabilitySuggestionsResponseDomainAvailableResponse {
                domain_available_response: self.domain_available_response,
            },
        )
    }
}
