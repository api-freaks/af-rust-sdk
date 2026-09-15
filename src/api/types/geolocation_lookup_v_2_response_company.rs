pub use crate::prelude::*;

/// Company or ISP information mapped to the IP address.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeolocationLookupV2ResponseCompany {
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

impl GeolocationLookupV2ResponseCompany {
    pub fn builder() -> GeolocationLookupV2ResponseCompanyBuilder {
        <GeolocationLookupV2ResponseCompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeolocationLookupV2ResponseCompanyBuilder {
    name: Option<String>,
    r#type: Option<String>,
    domain: Option<String>,
}

impl GeolocationLookupV2ResponseCompanyBuilder {
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

    /// Consumes the builder and constructs a [`GeolocationLookupV2ResponseCompany`].
    pub fn build(self) -> Result<GeolocationLookupV2ResponseCompany, BuildError> {
        Ok(GeolocationLookupV2ResponseCompany {
            name: self.name,
            r#type: self.r#type,
            domain: self.domain,
        })
    }
}
