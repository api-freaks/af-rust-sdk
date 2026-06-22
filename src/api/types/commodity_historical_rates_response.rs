pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CommodityHistoricalRatesResponse {
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
    pub metadata: HashMap<String, CommodityHistoricalRatesResponseMetadataValue>,
}

impl CommodityHistoricalRatesResponse {
    pub fn builder() -> CommodityHistoricalRatesResponseBuilder {
        <CommodityHistoricalRatesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityHistoricalRatesResponseBuilder {
    success: Option<bool>,
    timestamp: Option<f64>,
    rates: Option<HashMap<String, f64>>,
    metadata: Option<HashMap<String, CommodityHistoricalRatesResponseMetadataValue>>,
}

impl CommodityHistoricalRatesResponseBuilder {
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
        value: HashMap<String, CommodityHistoricalRatesResponseMetadataValue>,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommodityHistoricalRatesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](CommodityHistoricalRatesResponseBuilder::success)
    /// - [`timestamp`](CommodityHistoricalRatesResponseBuilder::timestamp)
    /// - [`rates`](CommodityHistoricalRatesResponseBuilder::rates)
    /// - [`metadata`](CommodityHistoricalRatesResponseBuilder::metadata)
    pub fn build(self) -> Result<CommodityHistoricalRatesResponse, BuildError> {
        Ok(CommodityHistoricalRatesResponse {
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
