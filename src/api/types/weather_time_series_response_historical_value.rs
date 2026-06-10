pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WeatherTimeSeriesResponseHistoricalValue {
    /// Daily historical data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily: Option<WeatherTimeSeriesResponseHistoricalValueDaily>,
    /// Hourly historical data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly: Option<Vec<WeatherTimeSeriesResponseHistoricalValueHourlyItem>>,
    /// Astronomy data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub astronomy: Option<WeatherTimeSeriesResponseHistoricalValueAstronomy>,
}

impl WeatherTimeSeriesResponseHistoricalValue {
    pub fn builder() -> WeatherTimeSeriesResponseHistoricalValueBuilder {
        <WeatherTimeSeriesResponseHistoricalValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WeatherTimeSeriesResponseHistoricalValueBuilder {
    daily: Option<WeatherTimeSeriesResponseHistoricalValueDaily>,
    hourly: Option<Vec<WeatherTimeSeriesResponseHistoricalValueHourlyItem>>,
    astronomy: Option<WeatherTimeSeriesResponseHistoricalValueAstronomy>,
}

impl WeatherTimeSeriesResponseHistoricalValueBuilder {
    pub fn daily(mut self, value: WeatherTimeSeriesResponseHistoricalValueDaily) -> Self {
        self.daily = Some(value);
        self
    }

    pub fn hourly(
        mut self,
        value: Vec<WeatherTimeSeriesResponseHistoricalValueHourlyItem>,
    ) -> Self {
        self.hourly = Some(value);
        self
    }

    pub fn astronomy(mut self, value: WeatherTimeSeriesResponseHistoricalValueAstronomy) -> Self {
        self.astronomy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WeatherTimeSeriesResponseHistoricalValue`].
    pub fn build(self) -> Result<WeatherTimeSeriesResponseHistoricalValue, BuildError> {
        Ok(WeatherTimeSeriesResponseHistoricalValue {
            daily: self.daily,
            hourly: self.hourly,
            astronomy: self.astronomy,
        })
    }
}
