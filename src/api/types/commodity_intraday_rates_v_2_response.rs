pub use crate::prelude::*;

/// Intraday response containing the prices of the requested symbol keyed by UTC timestamp.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommodityIntradayRatesV2Response {
    /// `true` for successful requests.
    #[serde(default)]
    pub success: bool,
    /// The symbol the prices belong to.
    #[serde(default)]
    pub symbol: String,
    /// The UTC day requested. Format: `YYYY-MM-DD`.
    #[serde(default)]
    pub date: String,
    /// The interval used, in lowercase form (`5M` comes back as `5m`).
    pub interval: CommodityIntradayRatesV2ResponseInterval,
    /// First mark of the window, `HH:mm` UTC.
    #[serde(rename = "startTime")]
    #[serde(default)]
    pub start_time: String,
    /// Number of points in `rates` (at most 100).
    #[serde(default)]
    pub count: i64,
    /// `HH:mm` value to pass as `startTime` for the next page, or `null` when the window reached the end of the day (or the current minute, for today).
    #[serde(rename = "nextStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_start_time: Option<String>,
    /// Prices keyed by ISO 8601 UTC timestamp (for example `2026-10-14T18:00:00Z`), in ascending order. Values are rounded to two decimals. A mark with no recorded price is left out rather than filled in.
    #[serde(default)]
    pub rates: HashMap<String, f64>,
    /// Unit and currency of the prices, keyed by the requested symbol. Omitted when the request sets `metadata=false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, CommodityIntradayRatesV2ResponseMetadataValue>>,
    /// Present when the symbol you sent was matched to a single alternative (200, with `resolvedTo`, for example `GOLD` answered as `XAU`), or not recognized at all (404, with a `suggestions` array when close matches exist).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unresolved: Option<HashMap<String, CommodityIntradayRatesV2ResponseUnresolvedValue>>,
}

impl CommodityIntradayRatesV2Response {
    pub fn builder() -> CommodityIntradayRatesV2ResponseBuilder {
        <CommodityIntradayRatesV2ResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityIntradayRatesV2ResponseBuilder {
    success: Option<bool>,
    symbol: Option<String>,
    date: Option<String>,
    interval: Option<CommodityIntradayRatesV2ResponseInterval>,
    start_time: Option<String>,
    count: Option<i64>,
    next_start_time: Option<String>,
    rates: Option<HashMap<String, f64>>,
    metadata: Option<HashMap<String, CommodityIntradayRatesV2ResponseMetadataValue>>,
    unresolved: Option<HashMap<String, CommodityIntradayRatesV2ResponseUnresolvedValue>>,
}

impl CommodityIntradayRatesV2ResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn symbol(mut self, value: impl Into<String>) -> Self {
        self.symbol = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn interval(mut self, value: CommodityIntradayRatesV2ResponseInterval) -> Self {
        self.interval = Some(value);
        self
    }

    pub fn start_time(mut self, value: impl Into<String>) -> Self {
        self.start_time = Some(value.into());
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn next_start_time(mut self, value: impl Into<String>) -> Self {
        self.next_start_time = Some(value.into());
        self
    }

    pub fn rates(mut self, value: HashMap<String, f64>) -> Self {
        self.rates = Some(value);
        self
    }

    pub fn metadata(
        mut self,
        value: HashMap<String, CommodityIntradayRatesV2ResponseMetadataValue>,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn unresolved(
        mut self,
        value: HashMap<String, CommodityIntradayRatesV2ResponseUnresolvedValue>,
    ) -> Self {
        self.unresolved = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommodityIntradayRatesV2Response`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](CommodityIntradayRatesV2ResponseBuilder::success)
    /// - [`symbol`](CommodityIntradayRatesV2ResponseBuilder::symbol)
    /// - [`date`](CommodityIntradayRatesV2ResponseBuilder::date)
    /// - [`interval`](CommodityIntradayRatesV2ResponseBuilder::interval)
    /// - [`start_time`](CommodityIntradayRatesV2ResponseBuilder::start_time)
    /// - [`count`](CommodityIntradayRatesV2ResponseBuilder::count)
    /// - [`rates`](CommodityIntradayRatesV2ResponseBuilder::rates)
    pub fn build(self) -> Result<CommodityIntradayRatesV2Response, BuildError> {
        Ok(CommodityIntradayRatesV2Response {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            symbol: self
                .symbol
                .ok_or_else(|| BuildError::missing_field("symbol"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            interval: self
                .interval
                .ok_or_else(|| BuildError::missing_field("interval"))?,
            start_time: self
                .start_time
                .ok_or_else(|| BuildError::missing_field("start_time"))?,
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
            next_start_time: self.next_start_time,
            rates: self
                .rates
                .ok_or_else(|| BuildError::missing_field("rates"))?,
            metadata: self.metadata,
            unresolved: self.unresolved,
        })
    }
}
