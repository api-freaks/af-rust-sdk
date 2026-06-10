pub use crate::prelude::*;

/// Query parameters for commodity_latest_rates
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CommodityLatestRatesQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the Response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<CommodityLatestRatesRequestFormat>,
    /// Comma separated list of desired commodities symbols *(e.g. XAU,XAG,WTI,BRENT)* **Required**
    #[serde(default)]
    pub symbols: Vec<Option<String>>,
    /// Exchange rates update period. Possible values are: (1) `10m` - 10 minute update (2) `1m` - 1 minute update **Required**
    pub updates: CommodityLatestRatesRequestUpdates,
    /// Specifies the target currency for the exchange rate; default quote currency is the market currency of commodity *(e.g. USD, EUR, INR)*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,
}

impl CommodityLatestRatesQueryRequest {
    pub fn builder() -> CommodityLatestRatesQueryRequestBuilder {
        <CommodityLatestRatesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityLatestRatesQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<CommodityLatestRatesRequestFormat>,
    symbols: Option<Vec<Option<String>>>,
    updates: Option<CommodityLatestRatesRequestUpdates>,
    quote: Option<String>,
}

impl CommodityLatestRatesQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: CommodityLatestRatesRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn symbols(mut self, value: Vec<Option<String>>) -> Self {
        self.symbols = Some(value);
        self
    }

    pub fn updates(mut self, value: CommodityLatestRatesRequestUpdates) -> Self {
        self.updates = Some(value);
        self
    }

    pub fn quote(mut self, value: impl Into<String>) -> Self {
        self.quote = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CommodityLatestRatesQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](CommodityLatestRatesQueryRequestBuilder::api_key)
    /// - [`symbols`](CommodityLatestRatesQueryRequestBuilder::symbols)
    /// - [`updates`](CommodityLatestRatesQueryRequestBuilder::updates)
    pub fn build(self) -> Result<CommodityLatestRatesQueryRequest, BuildError> {
        Ok(CommodityLatestRatesQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            symbols: self
                .symbols
                .ok_or_else(|| BuildError::missing_field("symbols"))?,
            updates: self
                .updates
                .ok_or_else(|| BuildError::missing_field("updates"))?,
            quote: self.quote,
        })
    }
}
