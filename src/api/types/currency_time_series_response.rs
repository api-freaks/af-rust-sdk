pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CurrencyTimeSeriesResponse {
    /// Starting date of the interval (provided via input).
    #[serde(rename = "startDate")]
    #[serde(default)]
    pub start_date: String,
    /// Ending date of the interval (provided via input).
    #[serde(rename = "endDate")]
    #[serde(default)]
    pub end_date: String,
    /// Base currency with respect to which all rates are calculated.
    #[serde(default)]
    pub base: String,
    /// List of historical exchange rates within the specified interval.
    #[serde(rename = "historicalRatesList")]
    #[serde(default)]
    pub historical_rates_list: Vec<CurrencyTimeSeriesResponseHistoricalRatesListItem>,
}

impl CurrencyTimeSeriesResponse {
    pub fn builder() -> CurrencyTimeSeriesResponseBuilder {
        <CurrencyTimeSeriesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrencyTimeSeriesResponseBuilder {
    start_date: Option<String>,
    end_date: Option<String>,
    base: Option<String>,
    historical_rates_list: Option<Vec<CurrencyTimeSeriesResponseHistoricalRatesListItem>>,
}

impl CurrencyTimeSeriesResponseBuilder {
    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
        self
    }

    pub fn base(mut self, value: impl Into<String>) -> Self {
        self.base = Some(value.into());
        self
    }

    pub fn historical_rates_list(
        mut self,
        value: Vec<CurrencyTimeSeriesResponseHistoricalRatesListItem>,
    ) -> Self {
        self.historical_rates_list = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CurrencyTimeSeriesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_date`](CurrencyTimeSeriesResponseBuilder::start_date)
    /// - [`end_date`](CurrencyTimeSeriesResponseBuilder::end_date)
    /// - [`base`](CurrencyTimeSeriesResponseBuilder::base)
    /// - [`historical_rates_list`](CurrencyTimeSeriesResponseBuilder::historical_rates_list)
    pub fn build(self) -> Result<CurrencyTimeSeriesResponse, BuildError> {
        Ok(CurrencyTimeSeriesResponse {
            start_date: self
                .start_date
                .ok_or_else(|| BuildError::missing_field("start_date"))?,
            end_date: self
                .end_date
                .ok_or_else(|| BuildError::missing_field("end_date"))?,
            base: self.base.ok_or_else(|| BuildError::missing_field("base"))?,
            historical_rates_list: self
                .historical_rates_list
                .ok_or_else(|| BuildError::missing_field("historical_rates_list"))?,
        })
    }
}
