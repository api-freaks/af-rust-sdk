pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkDomainAvailabilityCheckResponseBulkDomainAvailabilityResponseItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "domainAvailability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_availability: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
}

impl BulkDomainAvailabilityCheckResponseBulkDomainAvailabilityResponseItem {
    pub fn builder() -> BulkDomainAvailabilityCheckResponseBulkDomainAvailabilityResponseItemBuilder
    {
        <BulkDomainAvailabilityCheckResponseBulkDomainAvailabilityResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDomainAvailabilityCheckResponseBulkDomainAvailabilityResponseItemBuilder {
    domain: Option<String>,
    domain_availability: Option<bool>,
    status: Option<bool>,
}

impl BulkDomainAvailabilityCheckResponseBulkDomainAvailabilityResponseItemBuilder {
    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn domain_availability(mut self, value: bool) -> Self {
        self.domain_availability = Some(value);
        self
    }

    pub fn status(mut self, value: bool) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkDomainAvailabilityCheckResponseBulkDomainAvailabilityResponseItem`].
    pub fn build(
        self,
    ) -> Result<BulkDomainAvailabilityCheckResponseBulkDomainAvailabilityResponseItem, BuildError>
    {
        Ok(
            BulkDomainAvailabilityCheckResponseBulkDomainAvailabilityResponseItem {
                domain: self.domain,
                domain_availability: self.domain_availability,
                status: self.status,
            },
        )
    }
}
