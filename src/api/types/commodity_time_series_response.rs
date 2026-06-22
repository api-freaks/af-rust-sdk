pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CommodityTimeSeriesResponse {
    /// API request success indicator. "true" for successful requests.
    #[serde(default)]
    pub success: bool,
    /// Unix timestamp indicating when the response was generated.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub timestamp: f64,
    /// Map containing rate data for all the requested commodities.
    #[serde(default)]
    pub rates: HashMap<String, f64>,
    /// Map containing detailed information for all the requested commodities keyed by commodity symbol.
    #[serde(default)]
    pub metadata: HashMap<String, CommodityTimeSeriesResponseMetadataValue>,
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
    rates: Option<HashMap<String, f64>>,
    metadata: Option<HashMap<String, CommodityTimeSeriesResponseMetadataValue>>,
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

    pub fn rates(mut self, value: HashMap<String, f64>) -> Self {
        self.rates = Some(value);
        self
    }

    pub fn metadata(
        mut self,
        value: HashMap<String, CommodityTimeSeriesResponseMetadataValue>,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommodityTimeSeriesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](CommodityTimeSeriesResponseBuilder::success)
    /// - [`timestamp`](CommodityTimeSeriesResponseBuilder::timestamp)
    /// - [`rates`](CommodityTimeSeriesResponseBuilder::rates)
    /// - [`metadata`](CommodityTimeSeriesResponseBuilder::metadata)
    pub fn build(self) -> Result<CommodityTimeSeriesResponse, BuildError> {
        Ok(CommodityTimeSeriesResponse {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            timestamp: self
                .timestamp
                .ok_or_else(|| BuildError::missing_field("timestamp"))?,
            rates: self
                .rates
                .ok_or_else(|| BuildError::missing_field("rates"))?,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
        })
    }
}
