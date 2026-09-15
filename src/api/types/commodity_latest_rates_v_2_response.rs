pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CommodityLatestRatesV2Response {
    /// API request success indicator. "true" for successful requests.
    #[serde(default)]
    pub success: bool,
    /// Unix timestamp (seconds) indicating when the response was generated.
    #[serde(default)]
    pub timestamp: i64,
    /// Map of requested commodity symbols to their current live price.
    #[serde(default)]
    pub rates: HashMap<String, f64>,
    /// Map containing unit and quote currency metadata for all requested commodities, keyed by commodity symbol.
    #[serde(default)]
    pub metadata: HashMap<String, CommodityLatestRatesV2ResponseMetadataValue>,
    /// Present only when currency conversion for the requested `quote` is temporarily unavailable; rates are returned in each commodity's default currency instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
}

impl CommodityLatestRatesV2Response {
    pub fn builder() -> CommodityLatestRatesV2ResponseBuilder {
        <CommodityLatestRatesV2ResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityLatestRatesV2ResponseBuilder {
    success: Option<bool>,
    timestamp: Option<i64>,
    rates: Option<HashMap<String, f64>>,
    metadata: Option<HashMap<String, CommodityLatestRatesV2ResponseMetadataValue>>,
    warning: Option<String>,
}

impl CommodityLatestRatesV2ResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn timestamp(mut self, value: i64) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn rates(mut self, value: HashMap<String, f64>) -> Self {
        self.rates = Some(value);
        self
    }

    pub fn metadata(
        mut self,
        value: HashMap<String, CommodityLatestRatesV2ResponseMetadataValue>,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn warning(mut self, value: impl Into<String>) -> Self {
        self.warning = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CommodityLatestRatesV2Response`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](CommodityLatestRatesV2ResponseBuilder::success)
    /// - [`timestamp`](CommodityLatestRatesV2ResponseBuilder::timestamp)
    /// - [`rates`](CommodityLatestRatesV2ResponseBuilder::rates)
    /// - [`metadata`](CommodityLatestRatesV2ResponseBuilder::metadata)
    pub fn build(self) -> Result<CommodityLatestRatesV2Response, BuildError> {
        Ok(CommodityLatestRatesV2Response {
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
            warning: self.warning,
        })
    }
}
