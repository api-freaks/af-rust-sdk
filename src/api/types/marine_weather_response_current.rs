pub use crate::prelude::*;

/// Current marine data
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MarineWeatherResponseCurrent {
    /// ISO 8601 formatted timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub timestamp: Option<DateTime<FixedOffset>>,
    /// Significant height of combined sea waves (m)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wave_height: Option<f64>,
    /// Direction from which the combined waves are coming (°)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wave_direction: Option<f64>,
    /// Average period of combined sea waves (s)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wave_period: Option<f64>,
    /// Height of locally generated wind waves (m)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wind_wave_height: Option<f64>,
    /// Direction from which the wind waves are coming (°)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wind_wave_direction: Option<f64>,
    /// Average period of locally generated wind waves (s)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wind_wave_period: Option<f64>,
    /// Height of swell waves (m)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub swell_wave_height: Option<f64>,
    /// Direction from which the swell waves are coming (°)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub swell_wave_direction: Option<f64>,
    /// Average period of swell waves (s)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub swell_wave_period: Option<f64>,
    /// Sea level height relative to mean sea level (m)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub sea_level_height_msl: Option<f64>,
    /// Temperature of the ocean surface (°C)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub sea_surface_temperature: Option<f64>,
    /// Speed of the ocean current (km/h)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub ocean_current_velocity: Option<f64>,
    /// Direction of the ocean current (°)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub ocean_current_direction: Option<f64>,
}

impl MarineWeatherResponseCurrent {
    pub fn builder() -> MarineWeatherResponseCurrentBuilder {
        <MarineWeatherResponseCurrentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MarineWeatherResponseCurrentBuilder {
    timestamp: Option<DateTime<FixedOffset>>,
    wave_height: Option<f64>,
    wave_direction: Option<f64>,
    wave_period: Option<f64>,
    wind_wave_height: Option<f64>,
    wind_wave_direction: Option<f64>,
    wind_wave_period: Option<f64>,
    swell_wave_height: Option<f64>,
    swell_wave_direction: Option<f64>,
    swell_wave_period: Option<f64>,
    sea_level_height_msl: Option<f64>,
    sea_surface_temperature: Option<f64>,
    ocean_current_velocity: Option<f64>,
    ocean_current_direction: Option<f64>,
}

impl MarineWeatherResponseCurrentBuilder {
    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn wave_height(mut self, value: f64) -> Self {
        self.wave_height = Some(value);
        self
    }

    pub fn wave_direction(mut self, value: f64) -> Self {
        self.wave_direction = Some(value);
        self
    }

    pub fn wave_period(mut self, value: f64) -> Self {
        self.wave_period = Some(value);
        self
    }

    pub fn wind_wave_height(mut self, value: f64) -> Self {
        self.wind_wave_height = Some(value);
        self
    }

    pub fn wind_wave_direction(mut self, value: f64) -> Self {
        self.wind_wave_direction = Some(value);
        self
    }

    pub fn wind_wave_period(mut self, value: f64) -> Self {
        self.wind_wave_period = Some(value);
        self
    }

    pub fn swell_wave_height(mut self, value: f64) -> Self {
        self.swell_wave_height = Some(value);
        self
    }

    pub fn swell_wave_direction(mut self, value: f64) -> Self {
        self.swell_wave_direction = Some(value);
        self
    }

    pub fn swell_wave_period(mut self, value: f64) -> Self {
        self.swell_wave_period = Some(value);
        self
    }

    pub fn sea_level_height_msl(mut self, value: f64) -> Self {
        self.sea_level_height_msl = Some(value);
        self
    }

    pub fn sea_surface_temperature(mut self, value: f64) -> Self {
        self.sea_surface_temperature = Some(value);
        self
    }

    pub fn ocean_current_velocity(mut self, value: f64) -> Self {
        self.ocean_current_velocity = Some(value);
        self
    }

    pub fn ocean_current_direction(mut self, value: f64) -> Self {
        self.ocean_current_direction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MarineWeatherResponseCurrent`].
    pub fn build(self) -> Result<MarineWeatherResponseCurrent, BuildError> {
        Ok(MarineWeatherResponseCurrent {
            timestamp: self.timestamp,
            wave_height: self.wave_height,
            wave_direction: self.wave_direction,
            wave_period: self.wave_period,
            wind_wave_height: self.wind_wave_height,
            wind_wave_direction: self.wind_wave_direction,
            wind_wave_period: self.wind_wave_period,
            swell_wave_height: self.swell_wave_height,
            swell_wave_direction: self.swell_wave_direction,
            swell_wave_period: self.swell_wave_period,
            sea_level_height_msl: self.sea_level_height_msl,
            sea_surface_temperature: self.sea_surface_temperature,
            ocean_current_velocity: self.ocean_current_velocity,
            ocean_current_direction: self.ocean_current_direction,
        })
    }
}
