pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EmailValidateResponseAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<EmailValidateResponseAddressSecurity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<EmailValidateResponseAddressLocation>,
    #[serde(rename = "validIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_ip_address: Option<bool>,
}

impl EmailValidateResponseAddress {
    pub fn builder() -> EmailValidateResponseAddressBuilder {
        <EmailValidateResponseAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmailValidateResponseAddressBuilder {
    security: Option<EmailValidateResponseAddressSecurity>,
    location: Option<EmailValidateResponseAddressLocation>,
    valid_ip_address: Option<bool>,
}

impl EmailValidateResponseAddressBuilder {
    pub fn security(mut self, value: EmailValidateResponseAddressSecurity) -> Self {
        self.security = Some(value);
        self
    }

    pub fn location(mut self, value: EmailValidateResponseAddressLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn valid_ip_address(mut self, value: bool) -> Self {
        self.valid_ip_address = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EmailValidateResponseAddress`].
    pub fn build(self) -> Result<EmailValidateResponseAddress, BuildError> {
        Ok(EmailValidateResponseAddress {
            security: self.security,
            location: self.location,
            valid_ip_address: self.valid_ip_address,
        })
    }
}
