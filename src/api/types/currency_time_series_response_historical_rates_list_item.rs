pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CurrencyTimeSeriesResponseHistoricalRatesListItem {
    /// Date of the specific exchange rates.
    #[serde(default)]
    pub date: String,
    /// A map of currency symbols to their respective exchange rates for the given date.
    #[serde(default)]
    pub rates: HashMap<String, String>,
}

impl CurrencyTimeSeriesResponseHistoricalRatesListItem {
    pub fn builder() -> CurrencyTimeSeriesResponseHistoricalRatesListItemBuilder {
        <CurrencyTimeSeriesResponseHistoricalRatesListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrencyTimeSeriesResponseHistoricalRatesListItemBuilder {
    date: Option<String>,
    rates: Option<HashMap<String, String>>,
}

impl CurrencyTimeSeriesResponseHistoricalRatesListItemBuilder {
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn rates(mut self, value: HashMap<String, String>) -> Self {
        self.rates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CurrencyTimeSeriesResponseHistoricalRatesListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](CurrencyTimeSeriesResponseHistoricalRatesListItemBuilder::date)
    /// - [`rates`](CurrencyTimeSeriesResponseHistoricalRatesListItemBuilder::rates)
    pub fn build(self) -> Result<CurrencyTimeSeriesResponseHistoricalRatesListItem, BuildError> {
        Ok(CurrencyTimeSeriesResponseHistoricalRatesListItem {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            rates: self
                .rates
                .ok_or_else(|| BuildError::missing_field("rates"))?,
        })
    }
}
