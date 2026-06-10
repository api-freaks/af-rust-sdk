pub use crate::prelude::*;

/// Query parameters for commodity_historical_rates
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommodityHistoricalRatesQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<CommodityHistoricalRatesRequestFormat>,
    /// Historical date (YYYY-MM-DD)
    #[serde(default)]
    pub date: NaiveDate,
    /// Comma-separated list of commodity symbols
    #[serde(default)]
    pub symbols: Vec<Option<String>>,
}

impl CommodityHistoricalRatesQueryRequest {
    pub fn builder() -> CommodityHistoricalRatesQueryRequestBuilder {
        <CommodityHistoricalRatesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityHistoricalRatesQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<CommodityHistoricalRatesRequestFormat>,
    date: Option<NaiveDate>,
    symbols: Option<Vec<Option<String>>>,
}

impl CommodityHistoricalRatesQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: CommodityHistoricalRatesRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn symbols(mut self, value: Vec<Option<String>>) -> Self {
        self.symbols = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommodityHistoricalRatesQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](CommodityHistoricalRatesQueryRequestBuilder::api_key)
    /// - [`date`](CommodityHistoricalRatesQueryRequestBuilder::date)
    /// - [`symbols`](CommodityHistoricalRatesQueryRequestBuilder::symbols)
    pub fn build(self) -> Result<CommodityHistoricalRatesQueryRequest, BuildError> {
        Ok(CommodityHistoricalRatesQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            symbols: self
                .symbols
                .ok_or_else(|| BuildError::missing_field("symbols"))?,
        })
    }
}
