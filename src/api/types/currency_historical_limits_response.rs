pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CurrencyHistoricalLimitsResponse {
    /// Availability date range per currency, formatted "YYYY-MM-DD to YYYY-MM-DD".
    #[serde(rename = "availabilityPeriod")]
    #[serde(default)]
    pub availability_period: HashMap<String, String>,
}

impl CurrencyHistoricalLimitsResponse {
    pub fn builder() -> CurrencyHistoricalLimitsResponseBuilder {
        <CurrencyHistoricalLimitsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CurrencyHistoricalLimitsResponseBuilder {
    availability_period: Option<HashMap<String, String>>,
}

impl CurrencyHistoricalLimitsResponseBuilder {
    pub fn availability_period(mut self, value: HashMap<String, String>) -> Self {
        self.availability_period = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CurrencyHistoricalLimitsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`availability_period`](CurrencyHistoricalLimitsResponseBuilder::availability_period)
    pub fn build(self) -> Result<CurrencyHistoricalLimitsResponse, BuildError> {
        Ok(CurrencyHistoricalLimitsResponse {
            availability_period: self
                .availability_period
                .ok_or_else(|| BuildError::missing_field("availability_period"))?,
        })
    }
}
