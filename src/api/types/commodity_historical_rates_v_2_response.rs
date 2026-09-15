pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CommodityHistoricalRatesV2Response {
    /// API request success indicator. "true" for successful requests.
    #[serde(default)]
    pub success: bool,
    /// Date for which the user requested the commodity price. Format: YYYY-MM-DD.
    #[serde(default)]
    pub date: String,
    /// Map of requested commodity symbols to their OHLC price data on the given date.
    #[serde(default)]
    pub rates: HashMap<String, CommodityHistoricalRatesV2ResponseRatesValue>,
}

impl CommodityHistoricalRatesV2Response {
    pub fn builder() -> CommodityHistoricalRatesV2ResponseBuilder {
        <CommodityHistoricalRatesV2ResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityHistoricalRatesV2ResponseBuilder {
    success: Option<bool>,
    date: Option<String>,
    rates: Option<HashMap<String, CommodityHistoricalRatesV2ResponseRatesValue>>,
}

impl CommodityHistoricalRatesV2ResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn rates(
        mut self,
        value: HashMap<String, CommodityHistoricalRatesV2ResponseRatesValue>,
    ) -> Self {
        self.rates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommodityHistoricalRatesV2Response`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](CommodityHistoricalRatesV2ResponseBuilder::success)
    /// - [`date`](CommodityHistoricalRatesV2ResponseBuilder::date)
    /// - [`rates`](CommodityHistoricalRatesV2ResponseBuilder::rates)
    pub fn build(self) -> Result<CommodityHistoricalRatesV2Response, BuildError> {
        Ok(CommodityHistoricalRatesV2Response {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            rates: self
                .rates
                .ok_or_else(|| BuildError::missing_field("rates"))?,
        })
    }
}
