pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CommodityTimeSeriesResponse {
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
    pub metadata: Option<HashMap<String, CommodityTimeSeriesResponseMetadataValue>>,
    /// The start date of the time series data in YYYY-MM-DD format.
    #[serde(rename = "startDate")]
    #[serde(default)]
    pub start_date: String,
    /// The end date of the time series data in YYYY-MM-DD format.
    #[serde(rename = "endDate")]
    #[serde(default)]
    pub end_date: String,
    /// Date-indexed map; each key is a date (YYYY-MM-DD) whose value maps commodity symbols to OHLC data.
    #[serde(default)]
    pub rates: HashMap<String, HashMap<String, CommodityTimeSeriesResponseRatesValueValue>>,
}

impl CommodityTimeSeriesResponse {
    pub fn builder() -> CommodityTimeSeriesResponseBuilder {
        <CommodityTimeSeriesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityTimeSeriesResponseBuilder {
    success: Option<bool>,
    timestamp: Option<f64>,
    metadata: Option<HashMap<String, CommodityTimeSeriesResponseMetadataValue>>,
    start_date: Option<String>,
    end_date: Option<String>,
    rates: Option<HashMap<String, HashMap<String, CommodityTimeSeriesResponseRatesValueValue>>>,
}

impl CommodityTimeSeriesResponseBuilder {
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
        value: HashMap<String, CommodityTimeSeriesResponseMetadataValue>,
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

    pub fn rates(
        mut self,
        value: HashMap<String, HashMap<String, CommodityTimeSeriesResponseRatesValueValue>>,
    ) -> Self {
        self.rates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommodityTimeSeriesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](CommodityTimeSeriesResponseBuilder::success)
    /// - [`start_date`](CommodityTimeSeriesResponseBuilder::start_date)
    /// - [`end_date`](CommodityTimeSeriesResponseBuilder::end_date)
    /// - [`rates`](CommodityTimeSeriesResponseBuilder::rates)
    pub fn build(self) -> Result<CommodityTimeSeriesResponse, BuildError> {
        Ok(CommodityTimeSeriesResponse {
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
