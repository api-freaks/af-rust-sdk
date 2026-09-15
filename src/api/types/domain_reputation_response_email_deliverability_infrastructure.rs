pub use crate::prelude::*;

/// Mail server infrastructure backing the domain.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainReputationResponseEmailDeliverabilityInfrastructure {
    /// Number of MX records found for the domain.
    #[serde(default)]
    pub mx_count: i64,
    /// List of mail exchange server hostnames for the domain.
    #[serde(default)]
    pub mx_records: Vec<String>,
    /// Email hosting provider inferred from the MX records.
    #[serde(default)]
    pub mx_provider: String,
    /// Indicates whether the domain explicitly declines email via a null MX record.
    #[serde(default)]
    pub null_mx: bool,
}

impl DomainReputationResponseEmailDeliverabilityInfrastructure {
    pub fn builder() -> DomainReputationResponseEmailDeliverabilityInfrastructureBuilder {
        <DomainReputationResponseEmailDeliverabilityInfrastructureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainReputationResponseEmailDeliverabilityInfrastructureBuilder {
    mx_count: Option<i64>,
    mx_records: Option<Vec<String>>,
    mx_provider: Option<String>,
    null_mx: Option<bool>,
}

impl DomainReputationResponseEmailDeliverabilityInfrastructureBuilder {
    pub fn mx_count(mut self, value: i64) -> Self {
        self.mx_count = Some(value);
        self
    }

    pub fn mx_records(mut self, value: Vec<String>) -> Self {
        self.mx_records = Some(value);
        self
    }

    pub fn mx_provider(mut self, value: impl Into<String>) -> Self {
        self.mx_provider = Some(value.into());
        self
    }

    pub fn null_mx(mut self, value: bool) -> Self {
        self.null_mx = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainReputationResponseEmailDeliverabilityInfrastructure`].
    /// This method will fail if any of the following fields are not set:
    /// - [`mx_count`](DomainReputationResponseEmailDeliverabilityInfrastructureBuilder::mx_count)
    /// - [`mx_records`](DomainReputationResponseEmailDeliverabilityInfrastructureBuilder::mx_records)
    /// - [`mx_provider`](DomainReputationResponseEmailDeliverabilityInfrastructureBuilder::mx_provider)
    /// - [`null_mx`](DomainReputationResponseEmailDeliverabilityInfrastructureBuilder::null_mx)
    pub fn build(
        self,
    ) -> Result<DomainReputationResponseEmailDeliverabilityInfrastructure, BuildError> {
        Ok(DomainReputationResponseEmailDeliverabilityInfrastructure {
            mx_count: self
                .mx_count
                .ok_or_else(|| BuildError::missing_field("mx_count"))?,
            mx_records: self
                .mx_records
                .ok_or_else(|| BuildError::missing_field("mx_records"))?,
            mx_provider: self
                .mx_provider
                .ok_or_else(|| BuildError::missing_field("mx_provider"))?,
            null_mx: self
                .null_mx
                .ok_or_else(|| BuildError::missing_field("null_mx"))?,
        })
    }
}
