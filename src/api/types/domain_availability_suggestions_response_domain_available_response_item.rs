pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainAvailabilitySuggestionsResponseDomainAvailableResponseItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "domainAvailability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_availability: Option<bool>,
    /// Extra details if the domain is not registered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl DomainAvailabilitySuggestionsResponseDomainAvailableResponseItem {
    pub fn builder() -> DomainAvailabilitySuggestionsResponseDomainAvailableResponseItemBuilder {
        <DomainAvailabilitySuggestionsResponseDomainAvailableResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainAvailabilitySuggestionsResponseDomainAvailableResponseItemBuilder {
    domain: Option<String>,
    domain_availability: Option<bool>,
    message: Option<String>,
}

impl DomainAvailabilitySuggestionsResponseDomainAvailableResponseItemBuilder {
    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn domain_availability(mut self, value: bool) -> Self {
        self.domain_availability = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainAvailabilitySuggestionsResponseDomainAvailableResponseItem`].
    pub fn build(
        self,
    ) -> Result<DomainAvailabilitySuggestionsResponseDomainAvailableResponseItem, BuildError> {
        Ok(
            DomainAvailabilitySuggestionsResponseDomainAvailableResponseItem {
                domain: self.domain,
                domain_availability: self.domain_availability,
                message: self.message,
            },
        )
    }
}
