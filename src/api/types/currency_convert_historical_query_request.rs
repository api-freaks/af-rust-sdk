pub use crate::prelude::*;

/// Query parameters for currency_convert_historical
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CurrencyConvertHistoricalQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<CurrencyConvertHistoricalRequestFormat>,
    /// From currency symbol
    #[serde(default)]
    pub from: String,
    /// To currency symbol
    #[serde(default)]
    pub to: String,
    /// The Amount to be converted
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount: Option<f64>,
    /// specific date (format YYYY-MM-DD) of which exchange rates is used.
    #[serde(default)]
    pub date: NaiveDate,
}

impl CurrencyConvertHistoricalQueryRequest {
    pub fn builder() -> CurrencyConvertHistoricalQueryRequestBuilder {
        <CurrencyConvertHistoricalQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrencyConvertHistoricalQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<CurrencyConvertHistoricalRequestFormat>,
    from: Option<String>,
    to: Option<String>,
    amount: Option<f64>,
    date: Option<NaiveDate>,
}

impl CurrencyConvertHistoricalQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: CurrencyConvertHistoricalRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());
        self
    }

    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.to = Some(value.into());
        self
    }

    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CurrencyConvertHistoricalQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](CurrencyConvertHistoricalQueryRequestBuilder::api_key)
    /// - [`from`](CurrencyConvertHistoricalQueryRequestBuilder::from)
    /// - [`to`](CurrencyConvertHistoricalQueryRequestBuilder::to)
    /// - [`date`](CurrencyConvertHistoricalQueryRequestBuilder::date)
    pub fn build(self) -> Result<CurrencyConvertHistoricalQueryRequest, BuildError> {
        Ok(CurrencyConvertHistoricalQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
            amount: self.amount,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
        })
    }
}
