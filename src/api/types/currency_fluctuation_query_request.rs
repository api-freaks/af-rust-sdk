pub use crate::prelude::*;

/// Query parameters for currency_fluctuation
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CurrencyFluctuationQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<CurrencyFluctuationRequestFormat>,
    /// Start date (format YYYY-MM-DD) of the preferred time frame
    #[serde(rename = "startDate")]
    #[serde(default)]
    pub start_date: NaiveDate,
    /// End date (format YYYY-MM-DD) of the preferred time frame
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<NaiveDate>,
    /// Base currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    /// comma separated list of desired currencies/ commodities symbols
    #[serde(default)]
    pub symbols: Vec<Option<String>>,
}

impl CurrencyFluctuationQueryRequest {
    pub fn builder() -> CurrencyFluctuationQueryRequestBuilder {
        <CurrencyFluctuationQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrencyFluctuationQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<CurrencyFluctuationRequestFormat>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    base: Option<String>,
    symbols: Option<Vec<Option<String>>>,
}

impl CurrencyFluctuationQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: CurrencyFluctuationRequestFormat) -> Self {
        self.format = Some(value);
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

    pub fn base(mut self, value: impl Into<String>) -> Self {
        self.base = Some(value.into());
        self
    }

    pub fn symbols(mut self, value: Vec<Option<String>>) -> Self {
        self.symbols = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CurrencyFluctuationQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](CurrencyFluctuationQueryRequestBuilder::api_key)
    /// - [`start_date`](CurrencyFluctuationQueryRequestBuilder::start_date)
    /// - [`symbols`](CurrencyFluctuationQueryRequestBuilder::symbols)
    pub fn build(self) -> Result<CurrencyFluctuationQueryRequest, BuildError> {
        Ok(CurrencyFluctuationQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            start_date: self
                .start_date
                .ok_or_else(|| BuildError::missing_field("start_date"))?,
            end_date: self.end_date,
            base: self.base,
            symbols: self
                .symbols
                .ok_or_else(|| BuildError::missing_field("symbols"))?,
        })
    }
}
