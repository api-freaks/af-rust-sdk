pub use crate::prelude::*;

/// Query parameters for currency_symbols
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CurrencySymbolsQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<CurrencySymbolsRequestFormat>,
}

impl CurrencySymbolsQueryRequest {
    pub fn builder() -> CurrencySymbolsQueryRequestBuilder {
        <CurrencySymbolsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrencySymbolsQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<CurrencySymbolsRequestFormat>,
}

impl CurrencySymbolsQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: CurrencySymbolsRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CurrencySymbolsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](CurrencySymbolsQueryRequestBuilder::api_key)
    pub fn build(self) -> Result<CurrencySymbolsQueryRequest, BuildError> {
        Ok(CurrencySymbolsQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
        })
    }
}
