pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CommodityTimeSeriesV2ResponseRatesValueValue {
    /// Opening price on the given date. 0 for monthly-updated commodities.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub open: f64,
    /// Highest price on the given date. 0 for monthly-updated commodities.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub high: f64,
    /// Lowest price on the given date. 0 for monthly-updated commodities.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub low: f64,
    /// Closing price on the given date.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub close: f64,
}

impl CommodityTimeSeriesV2ResponseRatesValueValue {
    pub fn builder() -> CommodityTimeSeriesV2ResponseRatesValueValueBuilder {
        <CommodityTimeSeriesV2ResponseRatesValueValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityTimeSeriesV2ResponseRatesValueValueBuilder {
    open: Option<f64>,
    high: Option<f64>,
    low: Option<f64>,
    close: Option<f64>,
}

impl CommodityTimeSeriesV2ResponseRatesValueValueBuilder {
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

    /// Consumes the builder and constructs a [`CommodityTimeSeriesV2ResponseRatesValueValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`open`](CommodityTimeSeriesV2ResponseRatesValueValueBuilder::open)
    /// - [`high`](CommodityTimeSeriesV2ResponseRatesValueValueBuilder::high)
    /// - [`low`](CommodityTimeSeriesV2ResponseRatesValueValueBuilder::low)
    /// - [`close`](CommodityTimeSeriesV2ResponseRatesValueValueBuilder::close)
    pub fn build(self) -> Result<CommodityTimeSeriesV2ResponseRatesValueValue, BuildError> {
        Ok(CommodityTimeSeriesV2ResponseRatesValueValue {
            open: self.open.ok_or_else(|| BuildError::missing_field("open"))?,
            high: self.high.ok_or_else(|| BuildError::missing_field("high"))?,
            low: self.low.ok_or_else(|| BuildError::missing_field("low"))?,
            close: self
                .close
                .ok_or_else(|| BuildError::missing_field("close"))?,
        })
    }
}
