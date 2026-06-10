pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CommodityTimeSeriesResponseRatesValueValue {
    /// Opening price of the commodity on the given date.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub open: f64,
    /// Highest price of the commodity on the given date.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub high: f64,
    /// Lowest price of the commodity on the given date.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub low: f64,
    /// Closing price of the commodity on the given date.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub close: f64,
}

impl CommodityTimeSeriesResponseRatesValueValue {
    pub fn builder() -> CommodityTimeSeriesResponseRatesValueValueBuilder {
        <CommodityTimeSeriesResponseRatesValueValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityTimeSeriesResponseRatesValueValueBuilder {
    open: Option<f64>,
    high: Option<f64>,
    low: Option<f64>,
    close: Option<f64>,
}

impl CommodityTimeSeriesResponseRatesValueValueBuilder {
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

    /// Consumes the builder and constructs a [`CommodityTimeSeriesResponseRatesValueValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`open`](CommodityTimeSeriesResponseRatesValueValueBuilder::open)
    /// - [`high`](CommodityTimeSeriesResponseRatesValueValueBuilder::high)
    /// - [`low`](CommodityTimeSeriesResponseRatesValueValueBuilder::low)
    /// - [`close`](CommodityTimeSeriesResponseRatesValueValueBuilder::close)
    pub fn build(self) -> Result<CommodityTimeSeriesResponseRatesValueValue, BuildError> {
        Ok(CommodityTimeSeriesResponseRatesValueValue {
            open: self.open.ok_or_else(|| BuildError::missing_field("open"))?,
            high: self.high.ok_or_else(|| BuildError::missing_field("high"))?,
            low: self.low.ok_or_else(|| BuildError::missing_field("low"))?,
            close: self
                .close
                .ok_or_else(|| BuildError::missing_field("close"))?,
        })
    }
}
