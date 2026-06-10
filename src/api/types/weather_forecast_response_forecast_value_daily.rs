pub use crate::prelude::*;

/// Daily forecast data
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WeatherForecastResponseForecastValueDaily {
    /// ISO 8601 formatted timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub timestamp: Option<DateTime<FixedOffset>>,
    /// Weather condition code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weather_code: Option<i64>,
    /// Maximum air temperature at 2m (°C)
    #[serde(rename = "temperature_2m_max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub temperature2m_max: Option<f64>,
    /// Minimum air temperature at 2m (°C)
    #[serde(rename = "temperature_2m_min")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub temperature2m_min: Option<f64>,
    /// Mean air temperature at 2m (°C)
    #[serde(rename = "temperature_2m_mean")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub temperature2m_mean: Option<f64>,
    /// Maximum feels-like temperature (°C)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub apparent_temperature_max: Option<f64>,
    /// Minimum feels-like temperature (°C)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub apparent_temperature_min: Option<f64>,
    /// Mean feels-like temperature (°C)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub apparent_temperature_mean: Option<f64>,
    /// Daily maximum UV index
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub uv_index_max: Option<f64>,
    /// UV index clear sky max
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub uv_index_clear_sky_max: Option<f64>,
    /// Total rain (mm)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub rain_sum: Option<f64>,
    /// Total showers (mm)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub showers_sum: Option<f64>,
    /// Total snowfall (cm)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub snowfall_sum: Option<f64>,
    /// Total precipitation (mm)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub precipitation_sum: Option<f64>,
    /// Mean probability of precipitation (%)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precipitation_probability_mean: Option<i64>,
    /// Max wind speed at 10m (km/h)
    #[serde(rename = "wind_speed_10m_max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wind_speed10m_max: Option<f64>,
    /// Min wind speed at 10m (km/h)
    #[serde(rename = "wind_speed_10m_min")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wind_speed10m_min: Option<f64>,
    /// Mean wind speed at 10m (km/h)
    #[serde(rename = "wind_speed_10m_mean")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wind_speed10m_mean: Option<f64>,
    /// Max wind gusts at 10m (km/h)
    #[serde(rename = "wind_gusts_10m_max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wind_gusts10m_max: Option<f64>,
    /// Min wind gusts at 10m (km/h)
    #[serde(rename = "wind_gusts_10m_min")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wind_gusts10m_min: Option<f64>,
    /// Mean wind gusts at 10m (km/h)
    #[serde(rename = "wind_gusts_10m_mean")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wind_gusts10m_mean: Option<f64>,
    /// Dominant wind direction at 10m (°)
    #[serde(rename = "wind_direction_10m_dominant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_direction10m_dominant: Option<i64>,
    /// Total shortwave radiation (MJ/m²)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub shortwave_radiation_sum: Option<f64>,
    /// Mean surface pressure (hPa)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub surface_pressure_mean: Option<f64>,
    /// Mean sea-level pressure (hPa)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub pressure_msl_mean: Option<f64>,
    /// Mean visibility distance (m)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub visibility_mean: Option<f64>,
    /// Mean cloud cover (%)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_cover_mean: Option<i64>,
    /// Max dew point at 2m (°C)
    #[serde(rename = "dew_point_2m_max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub dew_point2m_max: Option<f64>,
    /// Min dew point at 2m (°C)
    #[serde(rename = "dew_point_2m_min")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub dew_point2m_min: Option<f64>,
    /// Mean dew point at 2m (°C)
    #[serde(rename = "dew_point_2m_mean")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub dew_point2m_mean: Option<f64>,
    /// Max relative humidity (%)
    #[serde(rename = "relative_humidity_2m_max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_humidity2m_max: Option<i64>,
    /// Min relative humidity (%)
    #[serde(rename = "relative_humidity_2m_min")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_humidity2m_min: Option<i64>,
    /// Mean relative humidity (%)
    #[serde(rename = "relative_humidity_2m_mean")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_humidity2m_mean: Option<i64>,
    /// ET₀ Reference Evapotranspiration (mm)
    #[serde(rename = "et0_fao_evapotranspiration_sum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub et0fao_evapotranspiration_sum: Option<f64>,
}

impl WeatherForecastResponseForecastValueDaily {
    pub fn builder() -> WeatherForecastResponseForecastValueDailyBuilder {
        <WeatherForecastResponseForecastValueDailyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WeatherForecastResponseForecastValueDailyBuilder {
    timestamp: Option<DateTime<FixedOffset>>,
    weather_code: Option<i64>,
    temperature2m_max: Option<f64>,
    temperature2m_min: Option<f64>,
    temperature2m_mean: Option<f64>,
    apparent_temperature_max: Option<f64>,
    apparent_temperature_min: Option<f64>,
    apparent_temperature_mean: Option<f64>,
    uv_index_max: Option<f64>,
    uv_index_clear_sky_max: Option<f64>,
    rain_sum: Option<f64>,
    showers_sum: Option<f64>,
    snowfall_sum: Option<f64>,
    precipitation_sum: Option<f64>,
    precipitation_probability_mean: Option<i64>,
    wind_speed10m_max: Option<f64>,
    wind_speed10m_min: Option<f64>,
    wind_speed10m_mean: Option<f64>,
    wind_gusts10m_max: Option<f64>,
    wind_gusts10m_min: Option<f64>,
    wind_gusts10m_mean: Option<f64>,
    wind_direction10m_dominant: Option<i64>,
    shortwave_radiation_sum: Option<f64>,
    surface_pressure_mean: Option<f64>,
    pressure_msl_mean: Option<f64>,
    visibility_mean: Option<f64>,
    cloud_cover_mean: Option<i64>,
    dew_point2m_max: Option<f64>,
    dew_point2m_min: Option<f64>,
    dew_point2m_mean: Option<f64>,
    relative_humidity2m_max: Option<i64>,
    relative_humidity2m_min: Option<i64>,
    relative_humidity2m_mean: Option<i64>,
    et0fao_evapotranspiration_sum: Option<f64>,
}

impl WeatherForecastResponseForecastValueDailyBuilder {
    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn weather_code(mut self, value: i64) -> Self {
        self.weather_code = Some(value);
        self
    }

    pub fn temperature2m_max(mut self, value: f64) -> Self {
        self.temperature2m_max = Some(value);
        self
    }

    pub fn temperature2m_min(mut self, value: f64) -> Self {
        self.temperature2m_min = Some(value);
        self
    }

    pub fn temperature2m_mean(mut self, value: f64) -> Self {
        self.temperature2m_mean = Some(value);
        self
    }

    pub fn apparent_temperature_max(mut self, value: f64) -> Self {
        self.apparent_temperature_max = Some(value);
        self
    }

    pub fn apparent_temperature_min(mut self, value: f64) -> Self {
        self.apparent_temperature_min = Some(value);
        self
    }

    pub fn apparent_temperature_mean(mut self, value: f64) -> Self {
        self.apparent_temperature_mean = Some(value);
        self
    }

    pub fn uv_index_max(mut self, value: f64) -> Self {
        self.uv_index_max = Some(value);
        self
    }

    pub fn uv_index_clear_sky_max(mut self, value: f64) -> Self {
        self.uv_index_clear_sky_max = Some(value);
        self
    }

    pub fn rain_sum(mut self, value: f64) -> Self {
        self.rain_sum = Some(value);
        self
    }

    pub fn showers_sum(mut self, value: f64) -> Self {
        self.showers_sum = Some(value);
        self
    }

    pub fn snowfall_sum(mut self, value: f64) -> Self {
        self.snowfall_sum = Some(value);
        self
    }

    pub fn precipitation_sum(mut self, value: f64) -> Self {
        self.precipitation_sum = Some(value);
        self
    }

    pub fn precipitation_probability_mean(mut self, value: i64) -> Self {
        self.precipitation_probability_mean = Some(value);
        self
    }

    pub fn wind_speed10m_max(mut self, value: f64) -> Self {
        self.wind_speed10m_max = Some(value);
        self
    }

    pub fn wind_speed10m_min(mut self, value: f64) -> Self {
        self.wind_speed10m_min = Some(value);
        self
    }

    pub fn wind_speed10m_mean(mut self, value: f64) -> Self {
        self.wind_speed10m_mean = Some(value);
        self
    }

    pub fn wind_gusts10m_max(mut self, value: f64) -> Self {
        self.wind_gusts10m_max = Some(value);
        self
    }

    pub fn wind_gusts10m_min(mut self, value: f64) -> Self {
        self.wind_gusts10m_min = Some(value);
        self
    }

    pub fn wind_gusts10m_mean(mut self, value: f64) -> Self {
        self.wind_gusts10m_mean = Some(value);
        self
    }

    pub fn wind_direction10m_dominant(mut self, value: i64) -> Self {
        self.wind_direction10m_dominant = Some(value);
        self
    }

    pub fn shortwave_radiation_sum(mut self, value: f64) -> Self {
        self.shortwave_radiation_sum = Some(value);
        self
    }

    pub fn surface_pressure_mean(mut self, value: f64) -> Self {
        self.surface_pressure_mean = Some(value);
        self
    }

    pub fn pressure_msl_mean(mut self, value: f64) -> Self {
        self.pressure_msl_mean = Some(value);
        self
    }

    pub fn visibility_mean(mut self, value: f64) -> Self {
        self.visibility_mean = Some(value);
        self
    }

    pub fn cloud_cover_mean(mut self, value: i64) -> Self {
        self.cloud_cover_mean = Some(value);
        self
    }

    pub fn dew_point2m_max(mut self, value: f64) -> Self {
        self.dew_point2m_max = Some(value);
        self
    }

    pub fn dew_point2m_min(mut self, value: f64) -> Self {
        self.dew_point2m_min = Some(value);
        self
    }

    pub fn dew_point2m_mean(mut self, value: f64) -> Self {
        self.dew_point2m_mean = Some(value);
        self
    }

    pub fn relative_humidity2m_max(mut self, value: i64) -> Self {
        self.relative_humidity2m_max = Some(value);
        self
    }

    pub fn relative_humidity2m_min(mut self, value: i64) -> Self {
        self.relative_humidity2m_min = Some(value);
        self
    }

    pub fn relative_humidity2m_mean(mut self, value: i64) -> Self {
        self.relative_humidity2m_mean = Some(value);
        self
    }

    pub fn et0fao_evapotranspiration_sum(mut self, value: f64) -> Self {
        self.et0fao_evapotranspiration_sum = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WeatherForecastResponseForecastValueDaily`].
    pub fn build(self) -> Result<WeatherForecastResponseForecastValueDaily, BuildError> {
        Ok(WeatherForecastResponseForecastValueDaily {
            timestamp: self.timestamp,
            weather_code: self.weather_code,
            temperature2m_max: self.temperature2m_max,
            temperature2m_min: self.temperature2m_min,
            temperature2m_mean: self.temperature2m_mean,
            apparent_temperature_max: self.apparent_temperature_max,
            apparent_temperature_min: self.apparent_temperature_min,
            apparent_temperature_mean: self.apparent_temperature_mean,
            uv_index_max: self.uv_index_max,
            uv_index_clear_sky_max: self.uv_index_clear_sky_max,
            rain_sum: self.rain_sum,
            showers_sum: self.showers_sum,
            snowfall_sum: self.snowfall_sum,
            precipitation_sum: self.precipitation_sum,
            precipitation_probability_mean: self.precipitation_probability_mean,
            wind_speed10m_max: self.wind_speed10m_max,
            wind_speed10m_min: self.wind_speed10m_min,
            wind_speed10m_mean: self.wind_speed10m_mean,
            wind_gusts10m_max: self.wind_gusts10m_max,
            wind_gusts10m_min: self.wind_gusts10m_min,
            wind_gusts10m_mean: self.wind_gusts10m_mean,
            wind_direction10m_dominant: self.wind_direction10m_dominant,
            shortwave_radiation_sum: self.shortwave_radiation_sum,
            surface_pressure_mean: self.surface_pressure_mean,
            pressure_msl_mean: self.pressure_msl_mean,
            visibility_mean: self.visibility_mean,
            cloud_cover_mean: self.cloud_cover_mean,
            dew_point2m_max: self.dew_point2m_max,
            dew_point2m_min: self.dew_point2m_min,
            dew_point2m_mean: self.dew_point2m_mean,
            relative_humidity2m_max: self.relative_humidity2m_max,
            relative_humidity2m_min: self.relative_humidity2m_min,
            relative_humidity2m_mean: self.relative_humidity2m_mean,
            et0fao_evapotranspiration_sum: self.et0fao_evapotranspiration_sum,
        })
    }
}
