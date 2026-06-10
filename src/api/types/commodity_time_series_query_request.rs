pub use crate::prelude::*;

/// Query parameters for commodity_time_series
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommodityTimeSeriesQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<CommodityTimeSeriesRequestFormat>,
    /// Comma-separated list of commodity symbols
    #[serde(default)]
    pub symbols: Vec<Option<String>>,
    /// Start date (YYYY-MM-DD)
    #[serde(rename = "startDate")]
    #[serde(default)]
    pub start_date: NaiveDate,
    /// End date (YYYY-MM-DD)
    #[serde(rename = "endDate")]
    #[serde(default)]
    pub end_date: NaiveDate,
}

impl CommodityTimeSeriesQueryRequest {
    pub fn builder() -> CommodityTimeSeriesQueryRequestBuilder {
        <CommodityTimeSeriesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityTimeSeriesQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<CommodityTimeSeriesRequestFormat>,
    symbols: Option<Vec<Option<String>>>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
}

impl CommodityTimeSeriesQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: CommodityTimeSeriesRequestFormat) -> Self {
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

    /// Consumes the builder and constructs a [`CommodityTimeSeriesQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](CommodityTimeSeriesQueryRequestBuilder::api_key)
    /// - [`symbols`](CommodityTimeSeriesQueryRequestBuilder::symbols)
    /// - [`start_date`](CommodityTimeSeriesQueryRequestBuilder::start_date)
    /// - [`end_date`](CommodityTimeSeriesQueryRequestBuilder::end_date)
    pub fn build(self) -> Result<CommodityTimeSeriesQueryRequest, BuildError> {
        Ok(CommodityTimeSeriesQueryRequest {
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
