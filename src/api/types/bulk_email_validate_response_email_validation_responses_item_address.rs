pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkEmailValidateResponseEmailValidationResponsesItemAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<BulkEmailValidateResponseEmailValidationResponsesItemAddressSecurity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<BulkEmailValidateResponseEmailValidationResponsesItemAddressLocation>,
    #[serde(rename = "validIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_ip_address: Option<bool>,
}

impl BulkEmailValidateResponseEmailValidationResponsesItemAddress {
    pub fn builder() -> BulkEmailValidateResponseEmailValidationResponsesItemAddressBuilder {
        <BulkEmailValidateResponseEmailValidationResponsesItemAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkEmailValidateResponseEmailValidationResponsesItemAddressBuilder {
    security: Option<BulkEmailValidateResponseEmailValidationResponsesItemAddressSecurity>,
    location: Option<BulkEmailValidateResponseEmailValidationResponsesItemAddressLocation>,
    valid_ip_address: Option<bool>,
}

impl BulkEmailValidateResponseEmailValidationResponsesItemAddressBuilder {
    pub fn security(
        mut self,
        value: BulkEmailValidateResponseEmailValidationResponsesItemAddressSecurity,
    ) -> Self {
        self.security = Some(value);
        self
    }

    pub fn location(
        mut self,
        value: BulkEmailValidateResponseEmailValidationResponsesItemAddressLocation,
    ) -> Self {
        self.location = Some(value);
        self
    }

    pub fn valid_ip_address(mut self, value: bool) -> Self {
        self.valid_ip_address = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkEmailValidateResponseEmailValidationResponsesItemAddress`].
    pub fn build(
        self,
    ) -> Result<BulkEmailValidateResponseEmailValidationResponsesItemAddress, BuildError> {
        Ok(
            BulkEmailValidateResponseEmailValidationResponsesItemAddress {
                security: self.security,
                location: self.location,
                valid_ip_address: self.valid_ip_address,
            },
        )
    }
}
