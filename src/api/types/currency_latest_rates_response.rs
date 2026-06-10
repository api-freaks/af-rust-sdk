pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CurrencyLatestRatesResponse {
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

impl CurrencyLatestRatesResponse {
    pub fn builder() -> CurrencyLatestRatesResponseBuilder {
        <CurrencyLatestRatesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrencyLatestRatesResponseBuilder {
    date: Option<String>,
    base: Option<String>,
    rates: Option<HashMap<String, String>>,
}

impl CurrencyLatestRatesResponseBuilder {
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

    /// Consumes the builder and constructs a [`CurrencyLatestRatesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](CurrencyLatestRatesResponseBuilder::date)
    /// - [`base`](CurrencyLatestRatesResponseBuilder::base)
    /// - [`rates`](CurrencyLatestRatesResponseBuilder::rates)
    pub fn build(self) -> Result<CurrencyLatestRatesResponse, BuildError> {
        Ok(CurrencyLatestRatesResponse {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            base: self.base.ok_or_else(|| BuildError::missing_field("base"))?,
            rates: self
                .rates
                .ok_or_else(|| BuildError::missing_field("rates"))?,
        })
    }
}
