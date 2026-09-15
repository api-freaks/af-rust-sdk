pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkEmailValidateResponseEmailResponseItemAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<BulkEmailValidateResponseEmailResponseItemAddressSecurity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<BulkEmailValidateResponseEmailResponseItemAddressLocation>,
    #[serde(rename = "validIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_ip_address: Option<bool>,
}

impl BulkEmailValidateResponseEmailResponseItemAddress {
    pub fn builder() -> BulkEmailValidateResponseEmailResponseItemAddressBuilder {
        <BulkEmailValidateResponseEmailResponseItemAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkEmailValidateResponseEmailResponseItemAddressBuilder {
    security: Option<BulkEmailValidateResponseEmailResponseItemAddressSecurity>,
    location: Option<BulkEmailValidateResponseEmailResponseItemAddressLocation>,
    valid_ip_address: Option<bool>,
}

impl BulkEmailValidateResponseEmailResponseItemAddressBuilder {
    pub fn security(
        mut self,
        value: BulkEmailValidateResponseEmailResponseItemAddressSecurity,
    ) -> Self {
        self.security = Some(value);
        self
    }

    pub fn location(
        mut self,
        value: BulkEmailValidateResponseEmailResponseItemAddressLocation,
    ) -> Self {
        self.location = Some(value);
        self
    }

    pub fn valid_ip_address(mut self, value: bool) -> Self {
        self.valid_ip_address = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkEmailValidateResponseEmailResponseItemAddress`].
    pub fn build(self) -> Result<BulkEmailValidateResponseEmailResponseItemAddress, BuildError> {
        Ok(BulkEmailValidateResponseEmailResponseItemAddress {
            security: self.security,
            location: self.location,
            valid_ip_address: self.valid_ip_address,
        })
    }
}
