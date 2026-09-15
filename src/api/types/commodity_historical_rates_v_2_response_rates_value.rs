pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CommodityHistoricalRatesV2ResponseRatesValue {
    /// Date for which prices were fetched (YYYY-MM-DD). May differ from the requested date when the API falls back to the last available rate before it, or snaps to the first day of the month for monthly-updated commodities.
    #[serde(default)]
    pub date: String,
    /// Opening price on the given date. 0 for monthly-updated commodities.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub open: f64,
    /// Highest price recorded on the given date. 0 for monthly-updated commodities.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub high: f64,
    /// Lowest price recorded on the given date. 0 for monthly-updated commodities.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub low: f64,
    /// Closing price on the given date.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub close: f64,
}

impl CommodityHistoricalRatesV2ResponseRatesValue {
    pub fn builder() -> CommodityHistoricalRatesV2ResponseRatesValueBuilder {
        <CommodityHistoricalRatesV2ResponseRatesValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityHistoricalRatesV2ResponseRatesValueBuilder {
    date: Option<String>,
    open: Option<f64>,
    high: Option<f64>,
    low: Option<f64>,
    close: Option<f64>,
}

impl CommodityHistoricalRatesV2ResponseRatesValueBuilder {
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn open(mut self, value: f64) -> Self {
        self.open = Some(value);
        self
    }

    pub fn high(mut self, value: f64) -> Self {
        self.high = Some(value);
        self
    }

    pub fn low(mut self, value: f64) -> Self {
        self.low = Some(value);
        self
    }

    pub fn close(mut self, value: f64) -> Self {
        self.close = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommodityHistoricalRatesV2ResponseRatesValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](CommodityHistoricalRatesV2ResponseRatesValueBuilder::date)
    /// - [`open`](CommodityHistoricalRatesV2ResponseRatesValueBuilder::open)
    /// - [`high`](CommodityHistoricalRatesV2ResponseRatesValueBuilder::high)
    /// - [`low`](CommodityHistoricalRatesV2ResponseRatesValueBuilder::low)
    /// - [`close`](CommodityHistoricalRatesV2ResponseRatesValueBuilder::close)
    pub fn build(self) -> Result<CommodityHistoricalRatesV2ResponseRatesValue, BuildError> {
        Ok(CommodityHistoricalRatesV2ResponseRatesValue {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            open: self.open.ok_or_else(|| BuildError::missing_field("open"))?,
            high: self.high.ok_or_else(|| BuildError::missing_field("high"))?,
            low: self.low.ok_or_else(|| BuildError::missing_field("low"))?,
            close: self
                .close
                .ok_or_else(|| BuildError::missing_field("close"))?,
        })
    }
}
