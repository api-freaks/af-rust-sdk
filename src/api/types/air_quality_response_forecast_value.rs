pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AirQualityResponseForecastValue {
    /// Hourly air quality forecast data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly: Option<Vec<AirQualityResponseForecastValueHourlyItem>>,
}

impl AirQualityResponseForecastValue {
    pub fn builder() -> AirQualityResponseForecastValueBuilder {
        <AirQualityResponseForecastValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AirQualityResponseForecastValueBuilder {
    hourly: Option<Vec<AirQualityResponseForecastValueHourlyItem>>,
}

impl AirQualityResponseForecastValueBuilder {
    pub fn hourly(mut self, value: Vec<AirQualityResponseForecastValueHourlyItem>) -> Self {
        self.hourly = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AirQualityResponseForecastValue`].
    pub fn build(self) -> Result<AirQualityResponseForecastValue, BuildError> {
        Ok(AirQualityResponseForecastValue {
            hourly: self.hourly,
        })
    }
}
