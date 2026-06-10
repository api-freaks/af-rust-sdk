pub use crate::prelude::*;

/// Historical data object of the provided date
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct HistoricalWeatherResponseHistorical {
    /// Daily historical data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily: Option<HistoricalWeatherResponseHistoricalDaily>,
    /// Hourly historical data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly: Option<Vec<HistoricalWeatherResponseHistoricalHourlyItem>>,
    /// Astronomy data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub astronomy: Option<HistoricalWeatherResponseHistoricalAstronomy>,
}

impl HistoricalWeatherResponseHistorical {
    pub fn builder() -> HistoricalWeatherResponseHistoricalBuilder {
        <HistoricalWeatherResponseHistoricalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HistoricalWeatherResponseHistoricalBuilder {
    daily: Option<HistoricalWeatherResponseHistoricalDaily>,
    hourly: Option<Vec<HistoricalWeatherResponseHistoricalHourlyItem>>,
    astronomy: Option<HistoricalWeatherResponseHistoricalAstronomy>,
}

impl HistoricalWeatherResponseHistoricalBuilder {
    pub fn daily(mut self, value: HistoricalWeatherResponseHistoricalDaily) -> Self {
        self.daily = Some(value);
        self
    }

    pub fn hourly(mut self, value: Vec<HistoricalWeatherResponseHistoricalHourlyItem>) -> Self {
        self.hourly = Some(value);
        self
    }

    pub fn astronomy(mut self, value: HistoricalWeatherResponseHistoricalAstronomy) -> Self {
        self.astronomy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`HistoricalWeatherResponseHistorical`].
    pub fn build(self) -> Result<HistoricalWeatherResponseHistorical, BuildError> {
        Ok(HistoricalWeatherResponseHistorical {
            daily: self.daily,
            hourly: self.hourly,
            astronomy: self.astronomy,
        })
    }
}
