pub use crate::prelude::*;

/// Domain eligibility information (populated for TLDs with registrant eligibility requirements, e.g. .eu).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactEligibilityInfo {
    /// Eligibility ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Eligibility name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Eligibility type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactEligibilityInfo {
    pub fn builder(
    ) -> BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactEligibilityInfoBuilder
    {
        <BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactEligibilityInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactEligibilityInfoBuilder {
    id: Option<String>,
    name: Option<String>,
    r#type: Option<String>,
}

impl BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactEligibilityInfoBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactEligibilityInfo`].
    pub fn build(
        self,
    ) -> Result<
        BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactEligibilityInfo,
        BuildError,
    > {
        Ok(
            BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactEligibilityInfo {
                id: self.id,
                name: self.name,
                r#type: self.r#type,
            },
        )
    }
}
