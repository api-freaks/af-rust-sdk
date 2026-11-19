pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkDomainAvailabilityCheckResponse {
    /// Deprecated: the API returns this field as `bulk_domain_availability_response`; read `bulk_domain_availability_response` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_domain_available_response:
        Option<Vec<BulkDomainAvailabilityCheckResponseBulkDomainAvailableResponseItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_domain_availability_response:
        Option<Vec<BulkDomainAvailabilityCheckResponseBulkDomainAvailabilityResponseItem>>,
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
    bulk_domain_availability_response:
        Option<Vec<BulkDomainAvailabilityCheckResponseBulkDomainAvailabilityResponseItem>>,
}

impl BulkDomainAvailabilityCheckResponseBuilder {
    pub fn bulk_domain_available_response(
        mut self,
        value: Vec<BulkDomainAvailabilityCheckResponseBulkDomainAvailableResponseItem>,
    ) -> Self {
        self.bulk_domain_available_response = Some(value);
        self
    }

    pub fn bulk_domain_availability_response(
        mut self,
        value: Vec<BulkDomainAvailabilityCheckResponseBulkDomainAvailabilityResponseItem>,
    ) -> Self {
        self.bulk_domain_availability_response = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkDomainAvailabilityCheckResponse`].
    pub fn build(self) -> Result<BulkDomainAvailabilityCheckResponse, BuildError> {
        Ok(BulkDomainAvailabilityCheckResponse {
            bulk_domain_available_response: self.bulk_domain_available_response,
            bulk_domain_availability_response: self.bulk_domain_availability_response,
        })
    }
}
