pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkDomainAvailabilityCheckResponseBulkDomainAvailableResponseItem {
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "domainAvailability")]
    #[serde(default)]
    pub domain_availability: bool,
    #[serde(default)]
    pub status: bool,
}

impl BulkDomainAvailabilityCheckResponseBulkDomainAvailableResponseItem {
    pub fn builder() -> BulkDomainAvailabilityCheckResponseBulkDomainAvailableResponseItemBuilder {
        <BulkDomainAvailabilityCheckResponseBulkDomainAvailableResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDomainAvailabilityCheckResponseBulkDomainAvailableResponseItemBuilder {
    domain: Option<String>,
    domain_availability: Option<bool>,
    status: Option<bool>,
}

impl BulkDomainAvailabilityCheckResponseBulkDomainAvailableResponseItemBuilder {
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

    /// Consumes the builder and constructs a [`BulkDomainAvailabilityCheckResponseBulkDomainAvailableResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain`](BulkDomainAvailabilityCheckResponseBulkDomainAvailableResponseItemBuilder::domain)
    /// - [`domain_availability`](BulkDomainAvailabilityCheckResponseBulkDomainAvailableResponseItemBuilder::domain_availability)
    /// - [`status`](BulkDomainAvailabilityCheckResponseBulkDomainAvailableResponseItemBuilder::status)
    pub fn build(
        self,
    ) -> Result<BulkDomainAvailabilityCheckResponseBulkDomainAvailableResponseItem, BuildError>
    {
        Ok(
            BulkDomainAvailabilityCheckResponseBulkDomainAvailableResponseItem {
                domain: self
                    .domain
                    .ok_or_else(|| BuildError::missing_field("domain"))?,
                domain_availability: self
                    .domain_availability
                    .ok_or_else(|| BuildError::missing_field("domain_availability"))?,
                status: self
                    .status
                    .ok_or_else(|| BuildError::missing_field("status"))?,
            },
        )
    }
}
