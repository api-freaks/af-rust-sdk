pub use crate::prelude::*;

/// The original request data provided by the client.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkPhoneValidateResponseItemRawInput {
    /// The phone number as entered by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    /// ISO-2 country code if provided in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// ISO-2 dialing origin country code if provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialer_region: Option<String>,
}

impl BulkPhoneValidateResponseItemRawInput {
    pub fn builder() -> BulkPhoneValidateResponseItemRawInputBuilder {
        <BulkPhoneValidateResponseItemRawInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkPhoneValidateResponseItemRawInputBuilder {
    number: Option<String>,
    region: Option<String>,
    dialer_region: Option<String>,
}

impl BulkPhoneValidateResponseItemRawInputBuilder {
    pub fn number(mut self, value: impl Into<String>) -> Self {
        self.number = Some(value.into());
        self
    }

    pub fn region(mut self, value: impl Into<String>) -> Self {
        self.region = Some(value.into());
        self
    }

    pub fn dialer_region(mut self, value: impl Into<String>) -> Self {
        self.dialer_region = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkPhoneValidateResponseItemRawInput`].
    pub fn build(self) -> Result<BulkPhoneValidateResponseItemRawInput, BuildError> {
        Ok(BulkPhoneValidateResponseItemRawInput {
            number: self.number,
            region: self.region,
            dialer_region: self.dialer_region,
        })
    }
}
