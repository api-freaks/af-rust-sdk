pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CommodityFluctuationResponseRatesValue {
    /// The price of the commodity on the start date of the interval.
    #[serde(rename = "startRate")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub start_rate: f64,
    /// The price of the commodity on the end date of the interval.
    #[serde(rename = "endRate")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub end_rate: f64,
    /// The absolute price difference between the end and start date. May be positive or negative.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub change: f64,
    /// The percentage change in price from start to end date. May be positive or negative.
    #[serde(rename = "changePercent")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub change_percent: f64,
}

impl CommodityFluctuationResponseRatesValue {
    pub fn builder() -> CommodityFluctuationResponseRatesValueBuilder {
        <CommodityFluctuationResponseRatesValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityFluctuationResponseRatesValueBuilder {
    start_rate: Option<f64>,
    end_rate: Option<f64>,
    change: Option<f64>,
    change_percent: Option<f64>,
}

impl CommodityFluctuationResponseRatesValueBuilder {
    pub fn start_rate(mut self, value: f64) -> Self {
        self.start_rate = Some(value);
        self
    }

    pub fn end_rate(mut self, value: f64) -> Self {
        self.end_rate = Some(value);
        self
    }

    pub fn change(mut self, value: f64) -> Self {
        self.change = Some(value);
        self
    }

    pub fn change_percent(mut self, value: f64) -> Self {
        self.change_percent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommodityFluctuationResponseRatesValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_rate`](CommodityFluctuationResponseRatesValueBuilder::start_rate)
    /// - [`end_rate`](CommodityFluctuationResponseRatesValueBuilder::end_rate)
    /// - [`change`](CommodityFluctuationResponseRatesValueBuilder::change)
    /// - [`change_percent`](CommodityFluctuationResponseRatesValueBuilder::change_percent)
    pub fn build(self) -> Result<CommodityFluctuationResponseRatesValue, BuildError> {
        Ok(CommodityFluctuationResponseRatesValue {
            start_rate: self
                .start_rate
                .ok_or_else(|| BuildError::missing_field("start_rate"))?,
            end_rate: self
                .end_rate
                .ok_or_else(|| BuildError::missing_field("end_rate"))?,
            change: self
                .change
                .ok_or_else(|| BuildError::missing_field("change"))?,
            change_percent: self
                .change_percent
                .ok_or_else(|| BuildError::missing_field("change_percent"))?,
        })
    }
}
