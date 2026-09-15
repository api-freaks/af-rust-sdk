pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CommodityTimeSeriesV2Response {
    /// API request success indicator. "true" for successful requests.
    #[serde(default)]
    pub success: bool,
    /// The start date of the time series data in YYYY-MM-DD format.
    #[serde(rename = "startDate")]
    #[serde(default)]
    pub start_date: String,
    /// The end date of the time series data in YYYY-MM-DD format.
    #[serde(rename = "endDate")]
    #[serde(default)]
    pub end_date: String,
    /// Map of trading dates (YYYY-MM-DD) to per-symbol OHLC data. Non-trading days are excluded.
    #[serde(default)]
    pub rates: HashMap<String, HashMap<String, CommodityTimeSeriesV2ResponseRatesValueValue>>,
}

impl CommodityTimeSeriesV2Response {
    pub fn builder() -> CommodityTimeSeriesV2ResponseBuilder {
        <CommodityTimeSeriesV2ResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityTimeSeriesV2ResponseBuilder {
    success: Option<bool>,
    start_date: Option<String>,
    end_date: Option<String>,
    rates: Option<HashMap<String, HashMap<String, CommodityTimeSeriesV2ResponseRatesValueValue>>>,
}

impl CommodityTimeSeriesV2ResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
        self
    }

    pub fn rates(
        mut self,
        value: HashMap<String, HashMap<String, CommodityTimeSeriesV2ResponseRatesValueValue>>,
    ) -> Self {
        self.rates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommodityTimeSeriesV2Response`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](CommodityTimeSeriesV2ResponseBuilder::success)
    /// - [`start_date`](CommodityTimeSeriesV2ResponseBuilder::start_date)
    /// - [`end_date`](CommodityTimeSeriesV2ResponseBuilder::end_date)
    /// - [`rates`](CommodityTimeSeriesV2ResponseBuilder::rates)
    pub fn build(self) -> Result<CommodityTimeSeriesV2Response, BuildError> {
        Ok(CommodityTimeSeriesV2Response {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            start_date: self
                .start_date
                .ok_or_else(|| BuildError::missing_field("start_date"))?,
            end_date: self
                .end_date
                .ok_or_else(|| BuildError::missing_field("end_date"))?,
            rates: self
                .rates
                .ok_or_else(|| BuildError::missing_field("rates"))?,
        })
    }
}
