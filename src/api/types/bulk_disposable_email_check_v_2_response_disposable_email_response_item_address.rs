pub use crate::prelude::*;

/// Geolocation and security enrichment data for the IP address.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddress {
    /// IP recognized/reachable by the geolocation provider.
    #[serde(rename = "validIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_ip_address: Option<bool>,
    /// Geolocation data derived from the IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location:
        Option<BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressLocation>,
    /// Threat and anonymity profile for the IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security:
        Option<BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressSecurity>,
}

impl BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddress {
    pub fn builder() -> BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressBuilder
    {
        <BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressBuilder {
    valid_ip_address: Option<bool>,
    location: Option<BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressLocation>,
    security: Option<BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressSecurity>,
}

impl BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressBuilder {
    pub fn valid_ip_address(mut self, value: bool) -> Self {
        self.valid_ip_address = Some(value);
        self
    }

    pub fn location(
        mut self,
        value: BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressLocation,
    ) -> Self {
        self.location = Some(value);
        self
    }

    pub fn security(
        mut self,
        value: BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressSecurity,
    ) -> Self {
        self.security = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddress`].
    pub fn build(
        self,
    ) -> Result<BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddress, BuildError>
    {
        Ok(
            BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddress {
                valid_ip_address: self.valid_ip_address,
                location: self.location,
                security: self.security,
            },
        )
    }
}
