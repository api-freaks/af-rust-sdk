pub use crate::prelude::*;

/// Query parameters for commodity_fluctuation_v2
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommodityFluctuationV2QueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Response format. Currently only `json` is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<CommodityFluctuationV2RequestFormat>,
    /// Comma-separated list of commodity symbols. Case-insensitive; duplicates are deduplicated server-side, with one response entry and one credit charge per unique symbol.
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

impl CommodityFluctuationV2QueryRequest {
    pub fn builder() -> CommodityFluctuationV2QueryRequestBuilder {
        <CommodityFluctuationV2QueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityFluctuationV2QueryRequestBuilder {
    api_key: Option<String>,
    format: Option<CommodityFluctuationV2RequestFormat>,
    symbols: Option<Vec<Option<String>>>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
}

impl CommodityFluctuationV2QueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: CommodityFluctuationV2RequestFormat) -> Self {
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

    /// Consumes the builder and constructs a [`CommodityFluctuationV2QueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](CommodityFluctuationV2QueryRequestBuilder::api_key)
    /// - [`symbols`](CommodityFluctuationV2QueryRequestBuilder::symbols)
    /// - [`start_date`](CommodityFluctuationV2QueryRequestBuilder::start_date)
    /// - [`end_date`](CommodityFluctuationV2QueryRequestBuilder::end_date)
    pub fn build(self) -> Result<CommodityFluctuationV2QueryRequest, BuildError> {
        Ok(CommodityFluctuationV2QueryRequest {
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
