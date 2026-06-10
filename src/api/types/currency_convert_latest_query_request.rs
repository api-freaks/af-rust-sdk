pub use crate::prelude::*;

/// Query parameters for currency_convert_latest
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CurrencyConvertLatestQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<CurrencyConvertLatestRequestFormat>,
    /// Source currency code
    #[serde(default)]
    pub from: String,
    /// Target currency code
    #[serde(default)]
    pub to: String,
    /// Amount to convert
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount: Option<f64>,
    /// Exchange rates update period (1d=daily, 1h=hourly, 10m=10 minutes, 1m=1 minute)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updates: Option<CurrencyConvertLatestRequestUpdates>,
}

impl CurrencyConvertLatestQueryRequest {
    pub fn builder() -> CurrencyConvertLatestQueryRequestBuilder {
        <CurrencyConvertLatestQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrencyConvertLatestQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<CurrencyConvertLatestRequestFormat>,
    from: Option<String>,
    to: Option<String>,
    amount: Option<f64>,
    updates: Option<CurrencyConvertLatestRequestUpdates>,
}

impl CurrencyConvertLatestQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: CurrencyConvertLatestRequestFormat) -> Self {
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

    pub fn updates(mut self, value: CurrencyConvertLatestRequestUpdates) -> Self {
        self.updates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CurrencyConvertLatestQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](CurrencyConvertLatestQueryRequestBuilder::api_key)
    /// - [`from`](CurrencyConvertLatestQueryRequestBuilder::from)
    /// - [`to`](CurrencyConvertLatestQueryRequestBuilder::to)
    pub fn build(self) -> Result<CurrencyConvertLatestQueryRequest, BuildError> {
        Ok(CurrencyConvertLatestQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
            amount: self.amount,
            updates: self.updates,
        })
    }
}
