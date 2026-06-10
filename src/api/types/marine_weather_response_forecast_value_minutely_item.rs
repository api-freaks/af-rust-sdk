pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MarineWeatherResponseForecastValueMinutelyItem {
    /// ISO 8601 formatted timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub timestamp: Option<DateTime<FixedOffset>>,
    /// Speed of ocean current (km/h)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub ocean_current_velocity: Option<f64>,
    /// Direction of ocean current (°)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocean_current_direction: Option<i64>,
    /// Sea level height relative to mean sea level (m)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub sea_level_height_msl: Option<f64>,
}

impl MarineWeatherResponseForecastValueMinutelyItem {
    pub fn builder() -> MarineWeatherResponseForecastValueMinutelyItemBuilder {
        <MarineWeatherResponseForecastValueMinutelyItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MarineWeatherResponseForecastValueMinutelyItemBuilder {
    timestamp: Option<DateTime<FixedOffset>>,
    ocean_current_velocity: Option<f64>,
    ocean_current_direction: Option<i64>,
    sea_level_height_msl: Option<f64>,
}

impl MarineWeatherResponseForecastValueMinutelyItemBuilder {
    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn ocean_current_velocity(mut self, value: f64) -> Self {
        self.ocean_current_velocity = Some(value);
        self
    }

    pub fn ocean_current_direction(mut self, value: i64) -> Self {
        self.ocean_current_direction = Some(value);
        self
    }

    pub fn sea_level_height_msl(mut self, value: f64) -> Self {
        self.sea_level_height_msl = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MarineWeatherResponseForecastValueMinutelyItem`].
    pub fn build(self) -> Result<MarineWeatherResponseForecastValueMinutelyItem, BuildError> {
        Ok(MarineWeatherResponseForecastValueMinutelyItem {
            timestamp: self.timestamp,
            ocean_current_velocity: self.ocean_current_velocity,
            ocean_current_direction: self.ocean_current_direction,
            sea_level_height_msl: self.sea_level_height_msl,
        })
    }
}
