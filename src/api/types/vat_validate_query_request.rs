pub use crate::prelude::*;

/// Query parameters for vat_validate
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatValidateQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Specify the desired response format. Options: 'json' (default) or 'xml'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<VatValidateRequestFormat>,
    /// EU or UK VAT number to validate.
    #[serde(rename = "vatNumber")]
    #[serde(default)]
    pub vat_number: String,
    /// Requester EU or UK VAT number.
    #[serde(rename = "requesterVatNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_vat_number: Option<String>,
}

impl VatValidateQueryRequest {
    pub fn builder() -> VatValidateQueryRequestBuilder {
        <VatValidateQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatValidateQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<VatValidateRequestFormat>,
    vat_number: Option<String>,
    requester_vat_number: Option<String>,
}

impl VatValidateQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: VatValidateRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn vat_number(mut self, value: impl Into<String>) -> Self {
        self.vat_number = Some(value.into());
        self
    }

    pub fn requester_vat_number(mut self, value: impl Into<String>) -> Self {
        self.requester_vat_number = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VatValidateQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](VatValidateQueryRequestBuilder::api_key)
    /// - [`vat_number`](VatValidateQueryRequestBuilder::vat_number)
    pub fn build(self) -> Result<VatValidateQueryRequest, BuildError> {
        Ok(VatValidateQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            vat_number: self
                .vat_number
                .ok_or_else(|| BuildError::missing_field("vat_number"))?,
            requester_vat_number: self.requester_vat_number,
        })
    }
}
