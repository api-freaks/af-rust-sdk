pub use crate::prelude::*;

/// Company or ISP information mapped to the IP address.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkGeolocationLookupV2ResponseItemAbuseCompany {
    /// Company name mapped to the IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Company category (ISP, HOSTING, BUSINESS, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Company domain name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl BulkGeolocationLookupV2ResponseItemAbuseCompany {
    pub fn builder() -> BulkGeolocationLookupV2ResponseItemAbuseCompanyBuilder {
        <BulkGeolocationLookupV2ResponseItemAbuseCompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupV2ResponseItemAbuseCompanyBuilder {
    name: Option<String>,
    r#type: Option<String>,
    domain: Option<String>,
}

impl BulkGeolocationLookupV2ResponseItemAbuseCompanyBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkGeolocationLookupV2ResponseItemAbuseCompany`].
    pub fn build(self) -> Result<BulkGeolocationLookupV2ResponseItemAbuseCompany, BuildError> {
        Ok(BulkGeolocationLookupV2ResponseItemAbuseCompany {
            name: self.name,
            r#type: self.r#type,
            domain: self.domain,
        })
    }
}
