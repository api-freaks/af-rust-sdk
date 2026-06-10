pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkGeolocationLookupResponseItemNetwork {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<BulkGeolocationLookupResponseItemNetworkAsn>,
    /// The type of the connection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<BulkGeolocationLookupResponseItemNetworkCompany>,
}

impl BulkGeolocationLookupResponseItemNetwork {
    pub fn builder() -> BulkGeolocationLookupResponseItemNetworkBuilder {
        <BulkGeolocationLookupResponseItemNetworkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupResponseItemNetworkBuilder {
    asn: Option<BulkGeolocationLookupResponseItemNetworkAsn>,
    connection_type: Option<String>,
    company: Option<BulkGeolocationLookupResponseItemNetworkCompany>,
}

impl BulkGeolocationLookupResponseItemNetworkBuilder {
    pub fn asn(mut self, value: BulkGeolocationLookupResponseItemNetworkAsn) -> Self {
        self.asn = Some(value);
        self
    }

    pub fn connection_type(mut self, value: impl Into<String>) -> Self {
        self.connection_type = Some(value.into());
        self
    }

    pub fn company(mut self, value: BulkGeolocationLookupResponseItemNetworkCompany) -> Self {
        self.company = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkGeolocationLookupResponseItemNetwork`].
    pub fn build(self) -> Result<BulkGeolocationLookupResponseItemNetwork, BuildError> {
        Ok(BulkGeolocationLookupResponseItemNetwork {
            asn: self.asn,
            connection_type: self.connection_type,
            company: self.company,
        })
    }
}
