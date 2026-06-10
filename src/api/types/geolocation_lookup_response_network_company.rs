pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeolocationLookupResponseNetworkCompany {
    /// The name of the company
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The type of the company
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The domain associated with the company
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl GeolocationLookupResponseNetworkCompany {
    pub fn builder() -> GeolocationLookupResponseNetworkCompanyBuilder {
        <GeolocationLookupResponseNetworkCompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeolocationLookupResponseNetworkCompanyBuilder {
    name: Option<String>,
    r#type: Option<String>,
    domain: Option<String>,
}

impl GeolocationLookupResponseNetworkCompanyBuilder {
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

    /// Consumes the builder and constructs a [`GeolocationLookupResponseNetworkCompany`].
    pub fn build(self) -> Result<GeolocationLookupResponseNetworkCompany, BuildError> {
        Ok(GeolocationLookupResponseNetworkCompany {
            name: self.name,
            r#type: self.r#type,
            domain: self.domain,
        })
    }
}
