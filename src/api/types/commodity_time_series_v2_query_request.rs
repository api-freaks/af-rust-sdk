pub use crate::prelude::*;

/// Query parameters for commodity_time_series_v2
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommodityTimeSeriesV2QueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Response format. Currently only `json` is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<CommodityTimeSeriesV2RequestFormat>,
    /// Comma-separated list of commodity symbols. Case-insensitive; duplicates are deduplicated server-side, with one response entry and one credit charge per unique symbol.
    #[serde(default)]
    pub symbols: Vec<Option<String>>,
    /// Start date (YYYY-MM-DD)
    #[serde(rename = "startDate")]
    #[serde(default)]
    pub start_date: NaiveDate,
    /// End date (YYYY-MM-DD). Maximum range is 365 days.
    #[serde(rename = "endDate")]
    #[serde(default)]
    pub end_date: NaiveDate,
}

impl CommodityTimeSeriesV2QueryRequest {
    pub fn builder() -> CommodityTimeSeriesV2QueryRequestBuilder {
        <CommodityTimeSeriesV2QueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityTimeSeriesV2QueryRequestBuilder {
    api_key: Option<String>,
    format: Option<CommodityTimeSeriesV2RequestFormat>,
    symbols: Option<Vec<Option<String>>>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
}

impl CommodityTimeSeriesV2QueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: CommodityTimeSeriesV2RequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn symbols(mut self, value: Vec<Option<String>>) -> Self {
        self.symbols = Some(value);
        self
    }

    pub fn start_date(mut self, value: NaiveDate) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn end_date(mut self, value: NaiveDate) -> Self {
        self.end_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommodityTimeSeriesV2QueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](CommodityTimeSeriesV2QueryRequestBuilder::api_key)
    /// - [`symbols`](CommodityTimeSeriesV2QueryRequestBuilder::symbols)
    /// - [`start_date`](CommodityTimeSeriesV2QueryRequestBuilder::start_date)
    /// - [`end_date`](CommodityTimeSeriesV2QueryRequestBuilder::end_date)
    pub fn build(self) -> Result<CommodityTimeSeriesV2QueryRequest, BuildError> {
        Ok(CommodityTimeSeriesV2QueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            symbols: self
                .symbols
                .ok_or_else(|| BuildError::missing_field("symbols"))?,
            start_date: self
                .start_date
                .ok_or_else(|| BuildError::missing_field("start_date"))?,
            end_date: self
                .end_date
                .ok_or_else(|| BuildError::missing_field("end_date"))?,
        })
    }
}
