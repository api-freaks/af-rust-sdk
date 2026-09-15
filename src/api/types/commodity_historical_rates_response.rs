pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CommodityHistoricalRatesResponse {
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
    pub metadata: Option<HashMap<String, CommodityHistoricalRatesResponseMetadataValue>>,
    /// Date for which the user requested the commodity price. Format: YYYY-MM-DD.
    #[serde(default)]
    pub date: String,
    /// Map containing rate data for each available requested commodity symbol, keyed by symbol.
    #[serde(default)]
    pub rates: HashMap<String, CommodityHistoricalRatesResponseRatesValue>,
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
    metadata: Option<HashMap<String, CommodityHistoricalRatesResponseMetadataValue>>,
    date: Option<String>,
    rates: Option<HashMap<String, CommodityHistoricalRatesResponseRatesValue>>,
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

    pub fn metadata(
        mut self,
        value: HashMap<String, CommodityHistoricalRatesResponseMetadataValue>,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn rates(
        mut self,
        value: HashMap<String, CommodityHistoricalRatesResponseRatesValue>,
    ) -> Self {
        self.rates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommodityHistoricalRatesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](CommodityHistoricalRatesResponseBuilder::success)
    /// - [`date`](CommodityHistoricalRatesResponseBuilder::date)
    /// - [`rates`](CommodityHistoricalRatesResponseBuilder::rates)
    pub fn build(self) -> Result<CommodityHistoricalRatesResponse, BuildError> {
        Ok(CommodityHistoricalRatesResponse {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            timestamp: self.timestamp,
            metadata: self.metadata,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            rates: self
                .rates
                .ok_or_else(|| BuildError::missing_field("rates"))?,
        })
    }
}
