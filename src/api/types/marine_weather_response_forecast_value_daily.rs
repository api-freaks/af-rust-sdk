pub use crate::prelude::*;

/// Daily marine forecast data
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MarineWeatherResponseForecastValueDaily {
    /// ISO 8601 formatted timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub timestamp: Option<DateTime<FixedOffset>>,
    /// Maximum significant wave height (m)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wave_height_max: Option<f64>,
    /// Dominant direction of waves (°)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wave_direction_dominant: Option<i64>,
    /// Maximum wave period (s)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wave_period_max: Option<f64>,
    /// Maximum wind-driven wave height (m)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wind_wave_height_max: Option<f64>,
    /// Dominant wind-wave direction (°)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_wave_direction_dominant: Option<i64>,
    /// Maximum wind-wave period (s)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wind_wave_period_max: Option<f64>,
    /// Maximum peak period of wind-driven waves (s)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wind_wave_peak_period_max: Option<f64>,
    /// Maximum swell wave height (m)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub swell_wave_height_max: Option<f64>,
    /// Dominant swell wave direction (°)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swell_wave_direction_dominant: Option<i64>,
    /// Maximum swell wave period (s)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub swell_wave_period_max: Option<f64>,
    /// Maximum peak period of swell waves (s)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub swell_wave_peak_period_max: Option<f64>,
}

impl MarineWeatherResponseForecastValueDaily {
    pub fn builder() -> MarineWeatherResponseForecastValueDailyBuilder {
        <MarineWeatherResponseForecastValueDailyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MarineWeatherResponseForecastValueDailyBuilder {
    timestamp: Option<DateTime<FixedOffset>>,
    wave_height_max: Option<f64>,
    wave_direction_dominant: Option<i64>,
    wave_period_max: Option<f64>,
    wind_wave_height_max: Option<f64>,
    wind_wave_direction_dominant: Option<i64>,
    wind_wave_period_max: Option<f64>,
    wind_wave_peak_period_max: Option<f64>,
    swell_wave_height_max: Option<f64>,
    swell_wave_direction_dominant: Option<i64>,
    swell_wave_period_max: Option<f64>,
    swell_wave_peak_period_max: Option<f64>,
}

impl MarineWeatherResponseForecastValueDailyBuilder {
    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn wave_height_max(mut self, value: f64) -> Self {
        self.wave_height_max = Some(value);
        self
    }

    pub fn wave_direction_dominant(mut self, value: i64) -> Self {
        self.wave_direction_dominant = Some(value);
        self
    }

    pub fn wave_period_max(mut self, value: f64) -> Self {
        self.wave_period_max = Some(value);
        self
    }

    pub fn wind_wave_height_max(mut self, value: f64) -> Self {
        self.wind_wave_height_max = Some(value);
        self
    }

    pub fn wind_wave_direction_dominant(mut self, value: i64) -> Self {
        self.wind_wave_direction_dominant = Some(value);
        self
    }

    pub fn wind_wave_period_max(mut self, value: f64) -> Self {
        self.wind_wave_period_max = Some(value);
        self
    }

    pub fn wind_wave_peak_period_max(mut self, value: f64) -> Self {
        self.wind_wave_peak_period_max = Some(value);
        self
    }

    pub fn swell_wave_height_max(mut self, value: f64) -> Self {
        self.swell_wave_height_max = Some(value);
        self
    }

    pub fn swell_wave_direction_dominant(mut self, value: i64) -> Self {
        self.swell_wave_direction_dominant = Some(value);
        self
    }

    pub fn swell_wave_period_max(mut self, value: f64) -> Self {
        self.swell_wave_period_max = Some(value);
        self
    }

    pub fn swell_wave_peak_period_max(mut self, value: f64) -> Self {
        self.swell_wave_peak_period_max = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MarineWeatherResponseForecastValueDaily`].
    pub fn build(self) -> Result<MarineWeatherResponseForecastValueDaily, BuildError> {
        Ok(MarineWeatherResponseForecastValueDaily {
            timestamp: self.timestamp,
            wave_height_max: self.wave_height_max,
            wave_direction_dominant: self.wave_direction_dominant,
            wave_period_max: self.wave_period_max,
            wind_wave_height_max: self.wind_wave_height_max,
            wind_wave_direction_dominant: self.wind_wave_direction_dominant,
            wind_wave_period_max: self.wind_wave_period_max,
            wind_wave_peak_period_max: self.wind_wave_peak_period_max,
            swell_wave_height_max: self.swell_wave_height_max,
            swell_wave_direction_dominant: self.swell_wave_direction_dominant,
            swell_wave_period_max: self.swell_wave_period_max,
            swell_wave_peak_period_max: self.swell_wave_peak_period_max,
        })
    }
}
