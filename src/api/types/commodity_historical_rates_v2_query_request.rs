pub use crate::prelude::*;

/// Query parameters for commodity_historical_rates_v2
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommodityHistoricalRatesV2QueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Response format. Currently only `json` is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<CommodityHistoricalRatesV2RequestFormat>,
    /// Comma-separated list of commodity symbols. Case-insensitive; duplicates are deduplicated server-side, with one response entry and one credit charge per unique symbol.
    #[serde(default)]
    pub symbols: Vec<Option<String>>,
    /// Date in YYYY-MM-DD format. Data available from 1990 onwards.
    #[serde(default)]
    pub date: NaiveDate,
}

impl CommodityHistoricalRatesV2QueryRequest {
    pub fn builder() -> CommodityHistoricalRatesV2QueryRequestBuilder {
        <CommodityHistoricalRatesV2QueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityHistoricalRatesV2QueryRequestBuilder {
    api_key: Option<String>,
    format: Option<CommodityHistoricalRatesV2RequestFormat>,
    symbols: Option<Vec<Option<String>>>,
    date: Option<NaiveDate>,
}

impl CommodityHistoricalRatesV2QueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: CommodityHistoricalRatesV2RequestFormat) -> Self {
        self.format = Some(value);
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

    /// Consumes the builder and constructs a [`CommodityHistoricalRatesV2QueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](CommodityHistoricalRatesV2QueryRequestBuilder::api_key)
    /// - [`symbols`](CommodityHistoricalRatesV2QueryRequestBuilder::symbols)
    /// - [`date`](CommodityHistoricalRatesV2QueryRequestBuilder::date)
    pub fn build(self) -> Result<CommodityHistoricalRatesV2QueryRequest, BuildError> {
        Ok(CommodityHistoricalRatesV2QueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            symbols: self
                .symbols
                .ok_or_else(|| BuildError::missing_field("symbols"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
        })
    }
}
