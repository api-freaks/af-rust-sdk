pub use crate::prelude::*;

/// Query parameters for currency_supported
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CurrencySupportedQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<CurrencySupportedRequestFormat>,
}

impl CurrencySupportedQueryRequest {
    pub fn builder() -> CurrencySupportedQueryRequestBuilder {
        <CurrencySupportedQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrencySupportedQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<CurrencySupportedRequestFormat>,
}

impl CurrencySupportedQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: CurrencySupportedRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CurrencySupportedQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](CurrencySupportedQueryRequestBuilder::api_key)
    pub fn build(self) -> Result<CurrencySupportedQueryRequest, BuildError> {
        Ok(CurrencySupportedQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
        })
    }
}
