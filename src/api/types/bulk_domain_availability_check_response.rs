pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkDomainAvailabilityCheckResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_domain_available_response:
        Option<Vec<BulkDomainAvailabilityCheckResponseBulkDomainAvailableResponseItem>>,
}

impl BulkDomainAvailabilityCheckResponse {
    pub fn builder() -> BulkDomainAvailabilityCheckResponseBuilder {
        <BulkDomainAvailabilityCheckResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDomainAvailabilityCheckResponseBuilder {
    bulk_domain_available_response:
        Option<Vec<BulkDomainAvailabilityCheckResponseBulkDomainAvailableResponseItem>>,
}

impl BulkDomainAvailabilityCheckResponseBuilder {
    pub fn bulk_domain_available_response(
        mut self,
        value: Vec<BulkDomainAvailabilityCheckResponseBulkDomainAvailableResponseItem>,
    ) -> Self {
        self.bulk_domain_available_response = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkDomainAvailabilityCheckResponse`].
    pub fn build(self) -> Result<BulkDomainAvailabilityCheckResponse, BuildError> {
        Ok(BulkDomainAvailabilityCheckResponse {
            bulk_domain_available_response: self.bulk_domain_available_response,
        })
    }
}
