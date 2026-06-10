pub use crate::prelude::*;

/// Query parameters for currency_latest_rates
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CurrencyLatestRatesQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<CurrencyLatestRatesRequestFormat>,
    /// Base currency for rate calculations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    /// Comma separated list of desired currency codes
    #[serde(default)]
    pub symbols: Vec<Option<String>>,
    /// Exchange rates update period (1d=daily, 1h=hourly, 10m=10 minutes, 1m=1 minute)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updates: Option<CurrencyLatestRatesRequestUpdates>,
}

impl CurrencyLatestRatesQueryRequest {
    pub fn builder() -> CurrencyLatestRatesQueryRequestBuilder {
        <CurrencyLatestRatesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrencyLatestRatesQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<CurrencyLatestRatesRequestFormat>,
    base: Option<String>,
    symbols: Option<Vec<Option<String>>>,
    updates: Option<CurrencyLatestRatesRequestUpdates>,
}

impl CurrencyLatestRatesQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: CurrencyLatestRatesRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn base(mut self, value: impl Into<String>) -> Self {
        self.base = Some(value.into());
        self
    }

    pub fn symbols(mut self, value: Vec<Option<String>>) -> Self {
        self.symbols = Some(value);
        self
    }

    pub fn updates(mut self, value: CurrencyLatestRatesRequestUpdates) -> Self {
        self.updates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CurrencyLatestRatesQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](CurrencyLatestRatesQueryRequestBuilder::api_key)
    /// - [`symbols`](CurrencyLatestRatesQueryRequestBuilder::symbols)
    pub fn build(self) -> Result<CurrencyLatestRatesQueryRequest, BuildError> {
        Ok(CurrencyLatestRatesQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            base: self.base,
            symbols: self
                .symbols
                .ok_or_else(|| BuildError::missing_field("symbols"))?,
            updates: self.updates,
        })
    }
}
