pub use crate::prelude::*;

/// Domain eligibility information (populated for TLDs with registrant eligibility requirements, e.g. .eu).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainWhoisLookupV2ResponseEligibilityInfo {
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

impl DomainWhoisLookupV2ResponseEligibilityInfo {
    pub fn builder() -> DomainWhoisLookupV2ResponseEligibilityInfoBuilder {
        <DomainWhoisLookupV2ResponseEligibilityInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainWhoisLookupV2ResponseEligibilityInfoBuilder {
    id: Option<String>,
    name: Option<String>,
    r#type: Option<String>,
}

impl DomainWhoisLookupV2ResponseEligibilityInfoBuilder {
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

    /// Consumes the builder and constructs a [`DomainWhoisLookupV2ResponseEligibilityInfo`].
    pub fn build(self) -> Result<DomainWhoisLookupV2ResponseEligibilityInfo, BuildError> {
        Ok(DomainWhoisLookupV2ResponseEligibilityInfo {
            id: self.id,
            name: self.name,
            r#type: self.r#type,
        })
    }
}
