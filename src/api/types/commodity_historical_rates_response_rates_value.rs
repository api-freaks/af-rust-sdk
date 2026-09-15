pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CommodityHistoricalRatesResponseRatesValue {
    /// Date for which commodity prices were fetched. Format: YYYY-MM-DD.
    #[serde(default)]
    pub date: String,
    /// The opening price of the commodity on the given date.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub open: f64,
    /// The highest price of the commodity recorded on the given date.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub high: f64,
    /// The lowest price of the commodity recorded on the given date.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub low: f64,
    /// The closing price of the commodity on the given date.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub close: f64,
}

impl CommodityHistoricalRatesResponseRatesValue {
    pub fn builder() -> CommodityHistoricalRatesResponseRatesValueBuilder {
        <CommodityHistoricalRatesResponseRatesValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityHistoricalRatesResponseRatesValueBuilder {
    date: Option<String>,
    open: Option<f64>,
    high: Option<f64>,
    low: Option<f64>,
    close: Option<f64>,
}

impl CommodityHistoricalRatesResponseRatesValueBuilder {
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

    /// Consumes the builder and constructs a [`CommodityHistoricalRatesResponseRatesValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](CommodityHistoricalRatesResponseRatesValueBuilder::date)
    /// - [`open`](CommodityHistoricalRatesResponseRatesValueBuilder::open)
    /// - [`high`](CommodityHistoricalRatesResponseRatesValueBuilder::high)
    /// - [`low`](CommodityHistoricalRatesResponseRatesValueBuilder::low)
    /// - [`close`](CommodityHistoricalRatesResponseRatesValueBuilder::close)
    pub fn build(self) -> Result<CommodityHistoricalRatesResponseRatesValue, BuildError> {
        Ok(CommodityHistoricalRatesResponseRatesValue {
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
