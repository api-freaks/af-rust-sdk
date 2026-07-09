pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeolocationLookupV2ResponseNetwork {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<GeolocationLookupV2ResponseNetworkAsn>,
    /// The type of the connection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<GeolocationLookupV2ResponseNetworkCompany>,
}

impl GeolocationLookupV2ResponseNetwork {
    pub fn builder() -> GeolocationLookupV2ResponseNetworkBuilder {
        <GeolocationLookupV2ResponseNetworkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeolocationLookupV2ResponseNetworkBuilder {
    asn: Option<GeolocationLookupV2ResponseNetworkAsn>,
    connection_type: Option<String>,
    company: Option<GeolocationLookupV2ResponseNetworkCompany>,
}

impl GeolocationLookupV2ResponseNetworkBuilder {
    pub fn asn(mut self, value: GeolocationLookupV2ResponseNetworkAsn) -> Self {
        self.asn = Some(value);
        self
    }

    pub fn connection_type(mut self, value: impl Into<String>) -> Self {
        self.connection_type = Some(value.into());
        self
    }

    pub fn company(mut self, value: GeolocationLookupV2ResponseNetworkCompany) -> Self {
        self.company = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeolocationLookupV2ResponseNetwork`].
    pub fn build(self) -> Result<GeolocationLookupV2ResponseNetwork, BuildError> {
        Ok(GeolocationLookupV2ResponseNetwork {
            asn: self.asn,
            connection_type: self.connection_type,
            company: self.company,
        })
    }
}
