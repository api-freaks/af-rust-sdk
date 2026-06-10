pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CurrencyHistoricalRatesResponse {
    /// For the latest currencies exchange rates endpoint, it is the date and time with timezone (UTC) at which these rates were recorded. For historical currencies exchange rates endpoint, it is the specific date in YYYY-MM-DD format.
    #[serde(default)]
    pub date: String,
    /// Base currency used for calculating exchange rates.
    #[serde(default)]
    pub base: String,
    /// A map of currency codes to their exchange rate against the base currency.
    #[serde(default)]
    pub rates: HashMap<String, String>,
}

impl CurrencyHistoricalRatesResponse {
    pub fn builder() -> CurrencyHistoricalRatesResponseBuilder {
        <CurrencyHistoricalRatesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrencyHistoricalRatesResponseBuilder {
    date: Option<String>,
    base: Option<String>,
    rates: Option<HashMap<String, String>>,
}

impl CurrencyHistoricalRatesResponseBuilder {
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn base(mut self, value: impl Into<String>) -> Self {
        self.base = Some(value.into());
        self
    }

    pub fn rates(mut self, value: HashMap<String, String>) -> Self {
        self.rates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CurrencyHistoricalRatesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](CurrencyHistoricalRatesResponseBuilder::date)
    /// - [`base`](CurrencyHistoricalRatesResponseBuilder::base)
    /// - [`rates`](CurrencyHistoricalRatesResponseBuilder::rates)
    pub fn build(self) -> Result<CurrencyHistoricalRatesResponse, BuildError> {
        Ok(CurrencyHistoricalRatesResponse {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            base: self.base.ok_or_else(|| BuildError::missing_field("base"))?,
            rates: self
                .rates
                .ok_or_else(|| BuildError::missing_field("rates"))?,
        })
    }
}
