pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FloodForecastResponseForecastValue {
    /// Daily flood forecast data for the date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily: Option<FloodForecastResponseForecastValueDaily>,
}

impl FloodForecastResponseForecastValue {
    pub fn builder() -> FloodForecastResponseForecastValueBuilder {
        <FloodForecastResponseForecastValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FloodForecastResponseForecastValueBuilder {
    daily: Option<FloodForecastResponseForecastValueDaily>,
}

impl FloodForecastResponseForecastValueBuilder {
    pub fn daily(mut self, value: FloodForecastResponseForecastValueDaily) -> Self {
        self.daily = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FloodForecastResponseForecastValue`].
    pub fn build(self) -> Result<FloodForecastResponseForecastValue, BuildError> {
        Ok(FloodForecastResponseForecastValue { daily: self.daily })
    }
}
