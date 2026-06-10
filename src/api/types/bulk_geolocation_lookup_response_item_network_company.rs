pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkGeolocationLookupResponseItemNetworkCompany {
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

impl BulkGeolocationLookupResponseItemNetworkCompany {
    pub fn builder() -> BulkGeolocationLookupResponseItemNetworkCompanyBuilder {
        <BulkGeolocationLookupResponseItemNetworkCompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupResponseItemNetworkCompanyBuilder {
    name: Option<String>,
    r#type: Option<String>,
    domain: Option<String>,
}

impl BulkGeolocationLookupResponseItemNetworkCompanyBuilder {
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

    /// Consumes the builder and constructs a [`BulkGeolocationLookupResponseItemNetworkCompany`].
    pub fn build(self) -> Result<BulkGeolocationLookupResponseItemNetworkCompany, BuildError> {
        Ok(BulkGeolocationLookupResponseItemNetworkCompany {
            name: self.name,
            r#type: self.r#type,
            domain: self.domain,
        })
    }
}
