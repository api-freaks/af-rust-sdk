pub use crate::prelude::*;

/// Query parameters for currency_historical_limits
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CurrencyHistoricalLimitsQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<CurrencyHistoricalLimitsRequestFormat>,
}

impl CurrencyHistoricalLimitsQueryRequest {
    pub fn builder() -> CurrencyHistoricalLimitsQueryRequestBuilder {
        <CurrencyHistoricalLimitsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrencyHistoricalLimitsQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<CurrencyHistoricalLimitsRequestFormat>,
}

impl CurrencyHistoricalLimitsQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: CurrencyHistoricalLimitsRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CurrencyHistoricalLimitsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](CurrencyHistoricalLimitsQueryRequestBuilder::api_key)
    pub fn build(self) -> Result<CurrencyHistoricalLimitsQueryRequest, BuildError> {
        Ok(CurrencyHistoricalLimitsQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
        })
    }
}
