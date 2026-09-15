pub use crate::prelude::*;

/// Query parameters for commodity_latest_rates_v2
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommodityLatestRatesV2QueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Response format. Currently only `json` is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<CommodityLatestRatesV2RequestFormat>,
    /// Comma-separated list of commodity symbols (e.g., XAU, WTIOIL-SPOT). Case-insensitive; duplicates are deduplicated server-side, with one response entry and one credit charge per unique symbol.
    #[serde(default)]
    pub symbols: Vec<Option<String>>,
    /// Target currency for the exchange rate. If omitted (or set to `default`), the default quote currency of each commodity is used. Requires a premium plan; ignored on lower-tier plans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,
}

impl CommodityLatestRatesV2QueryRequest {
    pub fn builder() -> CommodityLatestRatesV2QueryRequestBuilder {
        <CommodityLatestRatesV2QueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityLatestRatesV2QueryRequestBuilder {
    api_key: Option<String>,
    format: Option<CommodityLatestRatesV2RequestFormat>,
    symbols: Option<Vec<Option<String>>>,
    quote: Option<String>,
}

impl CommodityLatestRatesV2QueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: CommodityLatestRatesV2RequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn symbols(mut self, value: Vec<Option<String>>) -> Self {
        self.symbols = Some(value);
        self
    }

    pub fn quote(mut self, value: impl Into<String>) -> Self {
        self.quote = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CommodityLatestRatesV2QueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](CommodityLatestRatesV2QueryRequestBuilder::api_key)
    /// - [`symbols`](CommodityLatestRatesV2QueryRequestBuilder::symbols)
    pub fn build(self) -> Result<CommodityLatestRatesV2QueryRequest, BuildError> {
        Ok(CommodityLatestRatesV2QueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            symbols: self
                .symbols
                .ok_or_else(|| BuildError::missing_field("symbols"))?,
            quote: self.quote,
        })
    }
}
