pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CommodityFluctuationResponse {
    /// API request success indicator. "true" for successful requests.
    #[serde(default)]
    pub success: bool,
    /// Unix timestamp indicating when the response was generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub timestamp: Option<f64>,
    /// Map containing detailed information for all the requested commodities keyed by commodity symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, CommodityFluctuationResponseMetadataValue>>,
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
    pub rates: HashMap<String, CommodityFluctuationResponseRatesValue>,
}

impl CommodityFluctuationResponse {
    pub fn builder() -> CommodityFluctuationResponseBuilder {
        <CommodityFluctuationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityFluctuationResponseBuilder {
    success: Option<bool>,
    timestamp: Option<f64>,
    metadata: Option<HashMap<String, CommodityFluctuationResponseMetadataValue>>,
    start_date: Option<String>,
    end_date: Option<String>,
    rates: Option<HashMap<String, CommodityFluctuationResponseRatesValue>>,
}

impl CommodityFluctuationResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn timestamp(mut self, value: f64) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn metadata(
        mut self,
        value: HashMap<String, CommodityFluctuationResponseMetadataValue>,
    ) -> Self {
        self.metadata = Some(value);
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

    pub fn rates(mut self, value: HashMap<String, CommodityFluctuationResponseRatesValue>) -> Self {
        self.rates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommodityFluctuationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](CommodityFluctuationResponseBuilder::success)
    /// - [`start_date`](CommodityFluctuationResponseBuilder::start_date)
    /// - [`end_date`](CommodityFluctuationResponseBuilder::end_date)
    /// - [`rates`](CommodityFluctuationResponseBuilder::rates)
    pub fn build(self) -> Result<CommodityFluctuationResponse, BuildError> {
        Ok(CommodityFluctuationResponse {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            timestamp: self.timestamp,
            metadata: self.metadata,
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
