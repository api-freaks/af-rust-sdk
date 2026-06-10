pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeolocationLookupResponseNetwork {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<GeolocationLookupResponseNetworkAsn>,
    /// The type of the connection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<GeolocationLookupResponseNetworkCompany>,
}

impl GeolocationLookupResponseNetwork {
    pub fn builder() -> GeolocationLookupResponseNetworkBuilder {
        <GeolocationLookupResponseNetworkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeolocationLookupResponseNetworkBuilder {
    asn: Option<GeolocationLookupResponseNetworkAsn>,
    connection_type: Option<String>,
    company: Option<GeolocationLookupResponseNetworkCompany>,
}

impl GeolocationLookupResponseNetworkBuilder {
    pub fn asn(mut self, value: GeolocationLookupResponseNetworkAsn) -> Self {
        self.asn = Some(value);
        self
    }

    pub fn connection_type(mut self, value: impl Into<String>) -> Self {
        self.connection_type = Some(value.into());
        self
    }

    pub fn company(mut self, value: GeolocationLookupResponseNetworkCompany) -> Self {
        self.company = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeolocationLookupResponseNetwork`].
    pub fn build(self) -> Result<GeolocationLookupResponseNetwork, BuildError> {
        Ok(GeolocationLookupResponseNetwork {
            asn: self.asn,
            connection_type: self.connection_type,
            company: self.company,
        })
    }
}
