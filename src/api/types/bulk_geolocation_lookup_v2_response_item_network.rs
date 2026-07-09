pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkGeolocationLookupV2ResponseItemNetwork {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<BulkGeolocationLookupV2ResponseItemNetworkAsn>,
    /// The type of the connection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<BulkGeolocationLookupV2ResponseItemNetworkCompany>,
}

impl BulkGeolocationLookupV2ResponseItemNetwork {
    pub fn builder() -> BulkGeolocationLookupV2ResponseItemNetworkBuilder {
        <BulkGeolocationLookupV2ResponseItemNetworkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupV2ResponseItemNetworkBuilder {
    asn: Option<BulkGeolocationLookupV2ResponseItemNetworkAsn>,
    connection_type: Option<String>,
    company: Option<BulkGeolocationLookupV2ResponseItemNetworkCompany>,
}

impl BulkGeolocationLookupV2ResponseItemNetworkBuilder {
    pub fn asn(mut self, value: BulkGeolocationLookupV2ResponseItemNetworkAsn) -> Self {
        self.asn = Some(value);
        self
    }

    pub fn connection_type(mut self, value: impl Into<String>) -> Self {
        self.connection_type = Some(value.into());
        self
    }

    pub fn company(mut self, value: BulkGeolocationLookupV2ResponseItemNetworkCompany) -> Self {
        self.company = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkGeolocationLookupV2ResponseItemNetwork`].
    pub fn build(self) -> Result<BulkGeolocationLookupV2ResponseItemNetwork, BuildError> {
        Ok(BulkGeolocationLookupV2ResponseItemNetwork {
            asn: self.asn,
            connection_type: self.connection_type,
            company: self.company,
        })
    }
}
