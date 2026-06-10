pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CommodityLatestRatesResponse {
    /// API request success indicator. 'true' for successful requests.
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
    pub metadata: HashMap<String, CommodityLatestRatesResponseMetadataValue>,
}

impl CommodityLatestRatesResponse {
    pub fn builder() -> CommodityLatestRatesResponseBuilder {
        <CommodityLatestRatesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityLatestRatesResponseBuilder {
    success: Option<bool>,
    timestamp: Option<f64>,
    rates: Option<HashMap<String, f64>>,
    metadata: Option<HashMap<String, CommodityLatestRatesResponseMetadataValue>>,
}

impl CommodityLatestRatesResponseBuilder {
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
        value: HashMap<String, CommodityLatestRatesResponseMetadataValue>,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommodityLatestRatesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](CommodityLatestRatesResponseBuilder::success)
    /// - [`timestamp`](CommodityLatestRatesResponseBuilder::timestamp)
    /// - [`rates`](CommodityLatestRatesResponseBuilder::rates)
    /// - [`metadata`](CommodityLatestRatesResponseBuilder::metadata)
    pub fn build(self) -> Result<CommodityLatestRatesResponse, BuildError> {
        Ok(CommodityLatestRatesResponse {
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
