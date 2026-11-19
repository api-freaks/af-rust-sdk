pub use crate::prelude::*;

/// Query parameters for commodity_intraday_rates_v2
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CommodityIntradayRatesV2QueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Exactly one commodity symbol. Case-insensitive. Sending two or more returns a 400. Only symbols updated every second, every minute, or every 10 minutes have intraday rates; symbols updated hourly or less often return a 400. Use the Commodity Symbols endpoint for all valid values.
    #[serde(default)]
    pub symbols: String,
    /// Spacing between points. Case-insensitive. Symbols updated every 10 minutes accept only `10m`, `30m`, `1h`, `2h`, `4h`, `8h`, `12h`, `1d`.
    pub interval: CommodityIntradayRatesV2RequestInterval,
    /// The UTC day to read, in `YYYY-MM-DD` format. Cannot be in the future. For today, points stop at the current UTC minute.
    #[serde(default)]
    pub date: NaiveDate,
    /// First mark of the window, as a 24-hour UTC time in `HH:mm`. Must fall on a mark of the interval (for example, `14:30` works for `5m` or `30m`, but not for `1h`). For `1d` it can only be `00:00`.
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// Whether to include the `metadata` object. Pass `false` to receive only the prices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<bool>,
}

impl CommodityIntradayRatesV2QueryRequest {
    pub fn builder() -> CommodityIntradayRatesV2QueryRequestBuilder {
        <CommodityIntradayRatesV2QueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityIntradayRatesV2QueryRequestBuilder {
    api_key: Option<String>,
    symbols: Option<String>,
    interval: Option<CommodityIntradayRatesV2RequestInterval>,
    date: Option<NaiveDate>,
    start_time: Option<String>,
    metadata: Option<bool>,
}

impl CommodityIntradayRatesV2QueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn symbols(mut self, value: impl Into<String>) -> Self {
        self.symbols = Some(value.into());
        self
    }

    pub fn interval(mut self, value: CommodityIntradayRatesV2RequestInterval) -> Self {
        self.interval = Some(value);
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn start_time(mut self, value: impl Into<String>) -> Self {
        self.start_time = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: bool) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommodityIntradayRatesV2QueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](CommodityIntradayRatesV2QueryRequestBuilder::api_key)
    /// - [`symbols`](CommodityIntradayRatesV2QueryRequestBuilder::symbols)
    /// - [`interval`](CommodityIntradayRatesV2QueryRequestBuilder::interval)
    /// - [`date`](CommodityIntradayRatesV2QueryRequestBuilder::date)
    pub fn build(self) -> Result<CommodityIntradayRatesV2QueryRequest, BuildError> {
        Ok(CommodityIntradayRatesV2QueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            symbols: self
                .symbols
                .ok_or_else(|| BuildError::missing_field("symbols"))?,
            interval: self
                .interval
                .ok_or_else(|| BuildError::missing_field("interval"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            start_time: self.start_time,
            metadata: self.metadata,
        })
    }
}
