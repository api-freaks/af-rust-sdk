pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainAvailabilitySuggestionsResponse {
    #[serde(default)]
    pub domain_available_response:
        Vec<DomainAvailabilitySuggestionsResponseDomainAvailableResponseItem>,
}

impl DomainAvailabilitySuggestionsResponse {
    pub fn builder() -> DomainAvailabilitySuggestionsResponseBuilder {
        <DomainAvailabilitySuggestionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainAvailabilitySuggestionsResponseBuilder {
    domain_available_response:
        Option<Vec<DomainAvailabilitySuggestionsResponseDomainAvailableResponseItem>>,
}

impl DomainAvailabilitySuggestionsResponseBuilder {
    pub fn domain_available_response(
        mut self,
        value: Vec<DomainAvailabilitySuggestionsResponseDomainAvailableResponseItem>,
    ) -> Self {
        self.domain_available_response = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainAvailabilitySuggestionsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain_available_response`](DomainAvailabilitySuggestionsResponseBuilder::domain_available_response)
    pub fn build(self) -> Result<DomainAvailabilitySuggestionsResponse, BuildError> {
        Ok(DomainAvailabilitySuggestionsResponse {
            domain_available_response: self
                .domain_available_response
                .ok_or_else(|| BuildError::missing_field("domain_available_response"))?,
        })
    }
}
