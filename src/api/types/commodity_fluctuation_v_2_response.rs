pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CommodityFluctuationV2Response {
    /// API request success indicator. "true" for successful requests.
    #[serde(default)]
    pub success: bool,
    /// The start date of the fluctuation interval in YYYY-MM-DD format.
    #[serde(rename = "startDate")]
    #[serde(default)]
    pub start_date: String,
    /// The end date of the fluctuation interval in YYYY-MM-DD format.
    #[serde(rename = "endDate")]
    #[serde(default)]
    pub end_date: String,
    /// Map keyed by commodity symbol; value contains fluctuation metrics.
    #[serde(default)]
    pub rates: HashMap<String, CommodityFluctuationV2ResponseRatesValue>,
}

impl CommodityFluctuationV2Response {
    pub fn builder() -> CommodityFluctuationV2ResponseBuilder {
        <CommodityFluctuationV2ResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityFluctuationV2ResponseBuilder {
    success: Option<bool>,
    start_date: Option<String>,
    end_date: Option<String>,
    rates: Option<HashMap<String, CommodityFluctuationV2ResponseRatesValue>>,
}

impl CommodityFluctuationV2ResponseBuilder {
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
        value: HashMap<String, CommodityFluctuationV2ResponseRatesValue>,
    ) -> Self {
        self.rates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommodityFluctuationV2Response`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](CommodityFluctuationV2ResponseBuilder::success)
    /// - [`start_date`](CommodityFluctuationV2ResponseBuilder::start_date)
    /// - [`end_date`](CommodityFluctuationV2ResponseBuilder::end_date)
    /// - [`rates`](CommodityFluctuationV2ResponseBuilder::rates)
    pub fn build(self) -> Result<CommodityFluctuationV2Response, BuildError> {
        Ok(CommodityFluctuationV2Response {
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
