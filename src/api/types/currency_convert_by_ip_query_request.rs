pub use crate::prelude::*;

/// Query parameters for currency_convert_by_ip
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CurrencyConvertByIpQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<CurrencyConvertByIpRequestFormat>,
    /// Exchange rates update period (1d=daily, 1h=hourly, 10m=10 minutes, 1m=1 minute)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updates: Option<CurrencyConvertByIpRequestUpdates>,
    /// From currency symbol
    #[serde(default)]
    pub from: String,
    /// IPv4 or IPv6 geolocated currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Amount to convert
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount: Option<f64>,
}

impl CurrencyConvertByIpQueryRequest {
    pub fn builder() -> CurrencyConvertByIpQueryRequestBuilder {
        <CurrencyConvertByIpQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrencyConvertByIpQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<CurrencyConvertByIpRequestFormat>,
    updates: Option<CurrencyConvertByIpRequestUpdates>,
    from: Option<String>,
    ip: Option<String>,
    amount: Option<f64>,
}

impl CurrencyConvertByIpQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: CurrencyConvertByIpRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn updates(mut self, value: CurrencyConvertByIpRequestUpdates) -> Self {
        self.updates = Some(value);
        self
    }

    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());
        self
    }

    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CurrencyConvertByIpQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](CurrencyConvertByIpQueryRequestBuilder::api_key)
    /// - [`from`](CurrencyConvertByIpQueryRequestBuilder::from)
    pub fn build(self) -> Result<CurrencyConvertByIpQueryRequest, BuildError> {
        Ok(CurrencyConvertByIpQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            updates: self.updates,
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            ip: self.ip,
            amount: self.amount,
        })
    }
}
