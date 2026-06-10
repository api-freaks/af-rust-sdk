pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CurrencyConvertByIpResponse {
    /// Date and time with timezone (UTC) representing the exact time at which the conversion rate was recorded.
    #[serde(default)]
    pub date: String,
    /// Base currency code whose amount will be converted.
    #[serde(default)]
    pub from: String,
    /// Desired currency code for the converted amount.
    #[serde(default)]
    pub to: String,
    /// Current conversion rate with the base currency as the desired converted currency and the quote currency as the 'from' currency code.
    #[serde(default)]
    pub rate: String,
    /// IP Address whose country's currency will be extracted and used as 'to'. Defaults to the request IP if not provided.
    #[serde(rename = "ipAddress")]
    #[serde(default)]
    pub ip_address: String,
    /// The amount to be converted.
    #[serde(rename = "givenAmount")]
    #[serde(default)]
    pub given_amount: String,
    /// Converted amount in the desired currency.
    #[serde(rename = "convertedAmount")]
    #[serde(default)]
    pub converted_amount: String,
}

impl CurrencyConvertByIpResponse {
    pub fn builder() -> CurrencyConvertByIpResponseBuilder {
        <CurrencyConvertByIpResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrencyConvertByIpResponseBuilder {
    date: Option<String>,
    from: Option<String>,
    to: Option<String>,
    rate: Option<String>,
    ip_address: Option<String>,
    given_amount: Option<String>,
    converted_amount: Option<String>,
}

impl CurrencyConvertByIpResponseBuilder {
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
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

    pub fn rate(mut self, value: impl Into<String>) -> Self {
        self.rate = Some(value.into());
        self
    }

    pub fn ip_address(mut self, value: impl Into<String>) -> Self {
        self.ip_address = Some(value.into());
        self
    }

    pub fn given_amount(mut self, value: impl Into<String>) -> Self {
        self.given_amount = Some(value.into());
        self
    }

    pub fn converted_amount(mut self, value: impl Into<String>) -> Self {
        self.converted_amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CurrencyConvertByIpResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](CurrencyConvertByIpResponseBuilder::date)
    /// - [`from`](CurrencyConvertByIpResponseBuilder::from)
    /// - [`to`](CurrencyConvertByIpResponseBuilder::to)
    /// - [`rate`](CurrencyConvertByIpResponseBuilder::rate)
    /// - [`ip_address`](CurrencyConvertByIpResponseBuilder::ip_address)
    /// - [`given_amount`](CurrencyConvertByIpResponseBuilder::given_amount)
    /// - [`converted_amount`](CurrencyConvertByIpResponseBuilder::converted_amount)
    pub fn build(self) -> Result<CurrencyConvertByIpResponse, BuildError> {
        Ok(CurrencyConvertByIpResponse {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
            rate: self.rate.ok_or_else(|| BuildError::missing_field("rate"))?,
            ip_address: self
                .ip_address
                .ok_or_else(|| BuildError::missing_field("ip_address"))?,
            given_amount: self
                .given_amount
                .ok_or_else(|| BuildError::missing_field("given_amount"))?,
            converted_amount: self
                .converted_amount
                .ok_or_else(|| BuildError::missing_field("converted_amount"))?,
        })
    }
}
