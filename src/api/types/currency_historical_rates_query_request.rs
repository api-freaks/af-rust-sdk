pub use crate::prelude::*;

/// Query parameters for currency_historical_rates
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CurrencyHistoricalRatesQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<CurrencyHistoricalRatesRequestFormat>,
    /// Base currency for rate calculations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    /// Comma separated list of desired currency codes
    #[serde(default)]
    pub symbols: Vec<Option<String>>,
    /// Specific date in YYYY-MM-DD format
    #[serde(default)]
    pub date: NaiveDate,
}

impl CurrencyHistoricalRatesQueryRequest {
    pub fn builder() -> CurrencyHistoricalRatesQueryRequestBuilder {
        <CurrencyHistoricalRatesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrencyHistoricalRatesQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<CurrencyHistoricalRatesRequestFormat>,
    base: Option<String>,
    symbols: Option<Vec<Option<String>>>,
    date: Option<NaiveDate>,
}

impl CurrencyHistoricalRatesQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: CurrencyHistoricalRatesRequestFormat) -> Self {
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

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CurrencyHistoricalRatesQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](CurrencyHistoricalRatesQueryRequestBuilder::api_key)
    /// - [`symbols`](CurrencyHistoricalRatesQueryRequestBuilder::symbols)
    /// - [`date`](CurrencyHistoricalRatesQueryRequestBuilder::date)
    pub fn build(self) -> Result<CurrencyHistoricalRatesQueryRequest, BuildError> {
        Ok(CurrencyHistoricalRatesQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            base: self.base,
            symbols: self
                .symbols
                .ok_or_else(|| BuildError::missing_field("symbols"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
        })
    }
}
