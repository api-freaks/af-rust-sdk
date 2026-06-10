pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CurrencyFluctuationResponseRateFluctuationsValue {
    /// Starting rate of the currency during the interval.
    #[serde(rename = "startRate")]
    #[serde(default)]
    pub start_rate: String,
    /// Ending rate of the currency during the interval.
    #[serde(rename = "endRate")]
    #[serde(default)]
    pub end_rate: String,
    /// Absolute change in currency rate over the interval.
    #[serde(default)]
    pub change: String,
    /// Percentage change in currency rate over the interval.
    #[serde(rename = "percentChange")]
    #[serde(default)]
    pub percent_change: String,
}

impl CurrencyFluctuationResponseRateFluctuationsValue {
    pub fn builder() -> CurrencyFluctuationResponseRateFluctuationsValueBuilder {
        <CurrencyFluctuationResponseRateFluctuationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrencyFluctuationResponseRateFluctuationsValueBuilder {
    start_rate: Option<String>,
    end_rate: Option<String>,
    change: Option<String>,
    percent_change: Option<String>,
}

impl CurrencyFluctuationResponseRateFluctuationsValueBuilder {
    pub fn start_rate(mut self, value: impl Into<String>) -> Self {
        self.start_rate = Some(value.into());
        self
    }

    pub fn end_rate(mut self, value: impl Into<String>) -> Self {
        self.end_rate = Some(value.into());
        self
    }

    pub fn change(mut self, value: impl Into<String>) -> Self {
        self.change = Some(value.into());
        self
    }

    pub fn percent_change(mut self, value: impl Into<String>) -> Self {
        self.percent_change = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CurrencyFluctuationResponseRateFluctuationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_rate`](CurrencyFluctuationResponseRateFluctuationsValueBuilder::start_rate)
    /// - [`end_rate`](CurrencyFluctuationResponseRateFluctuationsValueBuilder::end_rate)
    /// - [`change`](CurrencyFluctuationResponseRateFluctuationsValueBuilder::change)
    /// - [`percent_change`](CurrencyFluctuationResponseRateFluctuationsValueBuilder::percent_change)
    pub fn build(self) -> Result<CurrencyFluctuationResponseRateFluctuationsValue, BuildError> {
        Ok(CurrencyFluctuationResponseRateFluctuationsValue {
            start_rate: self
                .start_rate
                .ok_or_else(|| BuildError::missing_field("start_rate"))?,
            end_rate: self
                .end_rate
                .ok_or_else(|| BuildError::missing_field("end_rate"))?,
            change: self
                .change
                .ok_or_else(|| BuildError::missing_field("change"))?,
            percent_change: self
                .percent_change
                .ok_or_else(|| BuildError::missing_field("percent_change"))?,
        })
    }
}
