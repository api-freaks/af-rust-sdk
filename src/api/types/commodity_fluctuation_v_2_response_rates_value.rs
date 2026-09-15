pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CommodityFluctuationV2ResponseRatesValue {
    /// Price of the commodity on the start date.
    #[serde(rename = "startRate")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub start_rate: f64,
    /// Price of the commodity on the end date.
    #[serde(rename = "endRate")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub end_rate: f64,
    /// Absolute price difference between end and start dates. May be negative.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub change: f64,
    /// Percentage price change from start to end date. May be negative.
    #[serde(rename = "changePercent")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub change_percent: f64,
}

impl CommodityFluctuationV2ResponseRatesValue {
    pub fn builder() -> CommodityFluctuationV2ResponseRatesValueBuilder {
        <CommodityFluctuationV2ResponseRatesValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityFluctuationV2ResponseRatesValueBuilder {
    start_rate: Option<f64>,
    end_rate: Option<f64>,
    change: Option<f64>,
    change_percent: Option<f64>,
}

impl CommodityFluctuationV2ResponseRatesValueBuilder {
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

    /// Consumes the builder and constructs a [`CommodityFluctuationV2ResponseRatesValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_rate`](CommodityFluctuationV2ResponseRatesValueBuilder::start_rate)
    /// - [`end_rate`](CommodityFluctuationV2ResponseRatesValueBuilder::end_rate)
    /// - [`change`](CommodityFluctuationV2ResponseRatesValueBuilder::change)
    /// - [`change_percent`](CommodityFluctuationV2ResponseRatesValueBuilder::change_percent)
    pub fn build(self) -> Result<CommodityFluctuationV2ResponseRatesValue, BuildError> {
        Ok(CommodityFluctuationV2ResponseRatesValue {
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
