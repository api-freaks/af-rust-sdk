pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CurrencyConvertHistoricalResponse {
    /// For the latest rates converter, it is the date and time with timezone (UTC) when the rate was recorded. For historical converter, it is the date provided in the input (format: YYYY-MM-DD).
    #[serde(default)]
    pub date: String,
    /// Base currency code (the currency being converted from).
    #[serde(default)]
    pub from: String,
    /// Desired currency code (the currency to convert to).
    #[serde(default)]
    pub to: String,
    /// Conversion rate from base currency to desired currency.
    #[serde(default)]
    pub rate: String,
    /// The amount in base currency to be converted.
    #[serde(rename = "givenAmount")]
    #[serde(default)]
    pub given_amount: String,
    /// The result of the conversion in the desired currency.
    #[serde(rename = "convertedAmount")]
    #[serde(default)]
    pub converted_amount: String,
}

impl CurrencyConvertHistoricalResponse {
    pub fn builder() -> CurrencyConvertHistoricalResponseBuilder {
        <CurrencyConvertHistoricalResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrencyConvertHistoricalResponseBuilder {
    date: Option<String>,
    from: Option<String>,
    to: Option<String>,
    rate: Option<String>,
    given_amount: Option<String>,
    converted_amount: Option<String>,
}

impl CurrencyConvertHistoricalResponseBuilder {
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

    pub fn given_amount(mut self, value: impl Into<String>) -> Self {
        self.given_amount = Some(value.into());
        self
    }

    pub fn converted_amount(mut self, value: impl Into<String>) -> Self {
        self.converted_amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CurrencyConvertHistoricalResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](CurrencyConvertHistoricalResponseBuilder::date)
    /// - [`from`](CurrencyConvertHistoricalResponseBuilder::from)
    /// - [`to`](CurrencyConvertHistoricalResponseBuilder::to)
    /// - [`rate`](CurrencyConvertHistoricalResponseBuilder::rate)
    /// - [`given_amount`](CurrencyConvertHistoricalResponseBuilder::given_amount)
    /// - [`converted_amount`](CurrencyConvertHistoricalResponseBuilder::converted_amount)
    pub fn build(self) -> Result<CurrencyConvertHistoricalResponse, BuildError> {
        Ok(CurrencyConvertHistoricalResponse {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
            rate: self.rate.ok_or_else(|| BuildError::missing_field("rate"))?,
            given_amount: self
                .given_amount
                .ok_or_else(|| BuildError::missing_field("given_amount"))?,
            converted_amount: self
                .converted_amount
                .ok_or_else(|| BuildError::missing_field("converted_amount"))?,
        })
    }
}
