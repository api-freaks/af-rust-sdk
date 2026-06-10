pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WeatherForecastResponseForecastValue {
    /// Daily forecast data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily: Option<WeatherForecastResponseForecastValueDaily>,
    /// Hourly forecast data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly: Option<Vec<WeatherForecastResponseForecastValueHourlyItem>>,
    /// Minutely forecast data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minutely: Option<Vec<WeatherForecastResponseForecastValueMinutelyItem>>,
    /// Astronomy data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub astronomy: Option<WeatherForecastResponseForecastValueAstronomy>,
}

impl WeatherForecastResponseForecastValue {
    pub fn builder() -> WeatherForecastResponseForecastValueBuilder {
        <WeatherForecastResponseForecastValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WeatherForecastResponseForecastValueBuilder {
    daily: Option<WeatherForecastResponseForecastValueDaily>,
    hourly: Option<Vec<WeatherForecastResponseForecastValueHourlyItem>>,
    minutely: Option<Vec<WeatherForecastResponseForecastValueMinutelyItem>>,
    astronomy: Option<WeatherForecastResponseForecastValueAstronomy>,
}

impl WeatherForecastResponseForecastValueBuilder {
    pub fn daily(mut self, value: WeatherForecastResponseForecastValueDaily) -> Self {
        self.daily = Some(value);
        self
    }

    pub fn hourly(mut self, value: Vec<WeatherForecastResponseForecastValueHourlyItem>) -> Self {
        self.hourly = Some(value);
        self
    }

    pub fn minutely(
        mut self,
        value: Vec<WeatherForecastResponseForecastValueMinutelyItem>,
    ) -> Self {
        self.minutely = Some(value);
        self
    }

    pub fn astronomy(mut self, value: WeatherForecastResponseForecastValueAstronomy) -> Self {
        self.astronomy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WeatherForecastResponseForecastValue`].
    pub fn build(self) -> Result<WeatherForecastResponseForecastValue, BuildError> {
        Ok(WeatherForecastResponseForecastValue {
            daily: self.daily,
            hourly: self.hourly,
            minutely: self.minutely,
            astronomy: self.astronomy,
        })
    }
}
