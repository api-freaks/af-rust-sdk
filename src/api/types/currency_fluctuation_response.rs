pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CurrencyFluctuationResponse {
    /// Starting date of the interval (provided via input).
    #[serde(rename = "startDate")]
    #[serde(default)]
    pub start_date: String,
    /// Ending date of the interval (provided via input).
    #[serde(rename = "endDate")]
    #[serde(default)]
    pub end_date: String,
    /// Base currency with respect to which all fluctuations are calculated.
    #[serde(default)]
    pub base: String,
    /// A map of currency symbols to their fluctuation details.
    #[serde(rename = "rateFluctuations")]
    #[serde(default)]
    pub rate_fluctuations: HashMap<String, CurrencyFluctuationResponseRateFluctuationsValue>,
}

impl CurrencyFluctuationResponse {
    pub fn builder() -> CurrencyFluctuationResponseBuilder {
        <CurrencyFluctuationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrencyFluctuationResponseBuilder {
    start_date: Option<String>,
    end_date: Option<String>,
    base: Option<String>,
    rate_fluctuations: Option<HashMap<String, CurrencyFluctuationResponseRateFluctuationsValue>>,
}

impl CurrencyFluctuationResponseBuilder {
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

    pub fn rate_fluctuations(
        mut self,
        value: HashMap<String, CurrencyFluctuationResponseRateFluctuationsValue>,
    ) -> Self {
        self.rate_fluctuations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CurrencyFluctuationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_date`](CurrencyFluctuationResponseBuilder::start_date)
    /// - [`end_date`](CurrencyFluctuationResponseBuilder::end_date)
    /// - [`base`](CurrencyFluctuationResponseBuilder::base)
    /// - [`rate_fluctuations`](CurrencyFluctuationResponseBuilder::rate_fluctuations)
    pub fn build(self) -> Result<CurrencyFluctuationResponse, BuildError> {
        Ok(CurrencyFluctuationResponse {
            start_date: self
                .start_date
                .ok_or_else(|| BuildError::missing_field("start_date"))?,
            end_date: self
                .end_date
                .ok_or_else(|| BuildError::missing_field("end_date"))?,
            base: self.base.ok_or_else(|| BuildError::missing_field("base"))?,
            rate_fluctuations: self
                .rate_fluctuations
                .ok_or_else(|| BuildError::missing_field("rate_fluctuations"))?,
        })
    }
}
