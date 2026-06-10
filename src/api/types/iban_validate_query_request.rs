pub use crate::prelude::*;

/// Query parameters for iban_validate
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IbanValidateQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Specify the desired response format. Options: 'json' (default) or 'xml'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<IbanValidateRequestFormat>,
    /// IBAN to validate.
    #[serde(default)]
    pub iban: String,
}

impl IbanValidateQueryRequest {
    pub fn builder() -> IbanValidateQueryRequestBuilder {
        <IbanValidateQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IbanValidateQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<IbanValidateRequestFormat>,
    iban: Option<String>,
}

impl IbanValidateQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: IbanValidateRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn iban(mut self, value: impl Into<String>) -> Self {
        self.iban = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IbanValidateQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](IbanValidateQueryRequestBuilder::api_key)
    /// - [`iban`](IbanValidateQueryRequestBuilder::iban)
    pub fn build(self) -> Result<IbanValidateQueryRequest, BuildError> {
        Ok(IbanValidateQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            iban: self.iban.ok_or_else(|| BuildError::missing_field("iban"))?,
        })
    }
}
