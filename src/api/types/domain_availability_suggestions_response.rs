pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum DomainAvailabilitySuggestionsResponse {
    DomainAvailabilitySuggestionsResponseDomain(DomainAvailabilitySuggestionsResponseDomain),

    DomainAvailabilitySuggestionsResponseDomainAvailableResponse(
        DomainAvailabilitySuggestionsResponseDomainAvailableResponse,
    ),
}

impl DomainAvailabilitySuggestionsResponse {
    pub fn is_domain_availability_suggestions_response_domain(&self) -> bool {
        matches!(self, Self::DomainAvailabilitySuggestionsResponseDomain(_))
    }

    pub fn is_domain_availability_suggestions_response_domain_available_response(&self) -> bool {
        matches!(
            self,
            Self::DomainAvailabilitySuggestionsResponseDomainAvailableResponse(_)
        )
    }

    pub fn as_domain_availability_suggestions_response_domain(
        &self,
    ) -> Option<&DomainAvailabilitySuggestionsResponseDomain> {
        match self {
            Self::DomainAvailabilitySuggestionsResponseDomain(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_domain_availability_suggestions_response_domain(
        self,
    ) -> Option<DomainAvailabilitySuggestionsResponseDomain> {
        match self {
            Self::DomainAvailabilitySuggestionsResponseDomain(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_domain_availability_suggestions_response_domain_available_response(
        &self,
    ) -> Option<&DomainAvailabilitySuggestionsResponseDomainAvailableResponse> {
        match self {
            Self::DomainAvailabilitySuggestionsResponseDomainAvailableResponse(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_domain_availability_suggestions_response_domain_available_response(
        self,
    ) -> Option<DomainAvailabilitySuggestionsResponseDomainAvailableResponse> {
        match self {
            Self::DomainAvailabilitySuggestionsResponseDomainAvailableResponse(value) => {
                Some(value)
            }
            _ => None,
        }
    }
}

impl fmt::Display for DomainAvailabilitySuggestionsResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DomainAvailabilitySuggestionsResponseDomain(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::DomainAvailabilitySuggestionsResponseDomainAvailableResponse(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
