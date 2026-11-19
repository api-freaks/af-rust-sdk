pub use crate::prelude::*;

/// Geolocation and security enrichment data for the IP address.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisposableEmailCheckV2ResponseAddress {
    /// IP recognized/reachable by the geolocation provider.
    #[serde(rename = "validIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_ip_address: Option<bool>,
    /// Geolocation data derived from the IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<DisposableEmailCheckV2ResponseAddressLocation>,
    /// Threat and anonymity profile for the IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<DisposableEmailCheckV2ResponseAddressSecurity>,
}

impl DisposableEmailCheckV2ResponseAddress {
    pub fn builder() -> DisposableEmailCheckV2ResponseAddressBuilder {
        <DisposableEmailCheckV2ResponseAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisposableEmailCheckV2ResponseAddressBuilder {
    valid_ip_address: Option<bool>,
    location: Option<DisposableEmailCheckV2ResponseAddressLocation>,
    security: Option<DisposableEmailCheckV2ResponseAddressSecurity>,
}

impl DisposableEmailCheckV2ResponseAddressBuilder {
    pub fn valid_ip_address(mut self, value: bool) -> Self {
        self.valid_ip_address = Some(value);
        self
    }

    pub fn location(mut self, value: DisposableEmailCheckV2ResponseAddressLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn security(mut self, value: DisposableEmailCheckV2ResponseAddressSecurity) -> Self {
        self.security = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DisposableEmailCheckV2ResponseAddress`].
    pub fn build(self) -> Result<DisposableEmailCheckV2ResponseAddress, BuildError> {
        Ok(DisposableEmailCheckV2ResponseAddress {
            valid_ip_address: self.valid_ip_address,
            location: self.location,
            security: self.security,
        })
    }
}
