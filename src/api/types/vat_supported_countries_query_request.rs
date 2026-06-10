pub use crate::prelude::*;

/// Query parameters for vat_supported_countries
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatSupportedCountriesQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response. Default is JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<VatSupportedCountriesRequestFormat>,
    /// Type of supported country. Supported values: IBAN, SWIFT, VAT. By default, it returns all supported countries for all types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<VatSupportedCountriesRequestType>,
}

impl VatSupportedCountriesQueryRequest {
    pub fn builder() -> VatSupportedCountriesQueryRequestBuilder {
        <VatSupportedCountriesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatSupportedCountriesQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<VatSupportedCountriesRequestFormat>,
    r#type: Option<VatSupportedCountriesRequestType>,
}

impl VatSupportedCountriesQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: VatSupportedCountriesRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn r#type(mut self, value: VatSupportedCountriesRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VatSupportedCountriesQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](VatSupportedCountriesQueryRequestBuilder::api_key)
    pub fn build(self) -> Result<VatSupportedCountriesQueryRequest, BuildError> {
        Ok(VatSupportedCountriesQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            r#type: self.r#type,
        })
    }
}
