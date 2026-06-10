pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MarineWeatherResponseForecastValue {
    /// Daily marine forecast data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily: Option<MarineWeatherResponseForecastValueDaily>,
    /// Hourly marine forecast data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly: Option<Vec<MarineWeatherResponseForecastValueHourlyItem>>,
    /// Minutely marine forecast data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minutely: Option<Vec<MarineWeatherResponseForecastValueMinutelyItem>>,
}

impl MarineWeatherResponseForecastValue {
    pub fn builder() -> MarineWeatherResponseForecastValueBuilder {
        <MarineWeatherResponseForecastValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MarineWeatherResponseForecastValueBuilder {
    daily: Option<MarineWeatherResponseForecastValueDaily>,
    hourly: Option<Vec<MarineWeatherResponseForecastValueHourlyItem>>,
    minutely: Option<Vec<MarineWeatherResponseForecastValueMinutelyItem>>,
}

impl MarineWeatherResponseForecastValueBuilder {
    pub fn daily(mut self, value: MarineWeatherResponseForecastValueDaily) -> Self {
        self.daily = Some(value);
        self
    }

    pub fn hourly(mut self, value: Vec<MarineWeatherResponseForecastValueHourlyItem>) -> Self {
        self.hourly = Some(value);
        self
    }

    pub fn minutely(mut self, value: Vec<MarineWeatherResponseForecastValueMinutelyItem>) -> Self {
        self.minutely = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MarineWeatherResponseForecastValue`].
    pub fn build(self) -> Result<MarineWeatherResponseForecastValue, BuildError> {
        Ok(MarineWeatherResponseForecastValue {
            daily: self.daily,
            hourly: self.hourly,
            minutely: self.minutely,
        })
    }
}
