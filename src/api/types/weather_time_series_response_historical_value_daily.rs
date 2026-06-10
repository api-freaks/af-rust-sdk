pub use crate::prelude::*;

/// Daily historical data
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WeatherTimeSeriesResponseHistoricalValueDaily {
    /// ISO 8601 formatted timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub timestamp: Option<DateTime<FixedOffset>>,
    /// Weather condition code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weather_code: Option<i64>,
    /// Daily mean air temperature at 2 meters (°C)
    #[serde(rename = "temperature_2m_mean")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub temperature2m_mean: Option<f64>,
    /// Daily maximum air temperature at 2 meters (°C)
    #[serde(rename = "temperature_2m_max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub temperature2m_max: Option<f64>,
    /// Daily minimum air temperature at 2 meters (°C)
    #[serde(rename = "temperature_2m_min")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub temperature2m_min: Option<f64>,
    /// Daily mean perceived temperature (°C)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub apparent_temperature_mean: Option<f64>,
    /// Daily maximum perceived temperature (°C)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub apparent_temperature_max: Option<f64>,
    /// Daily minimum perceived temperature (°C)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub apparent_temperature_min: Option<f64>,
    /// Total precipitation (mm)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub precipitation_sum: Option<f64>,
    /// Total rainfall (mm)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub rain_sum: Option<f64>,
    /// Total snowfall (cm)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub snowfall_sum: Option<f64>,
    /// Maximum wind speed at 10 meters (km/h)
    #[serde(rename = "wind_speed_10m_max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wind_speed10m_max: Option<f64>,
    /// Maximum wind gusts at 10 meters (km/h)
    #[serde(rename = "wind_gusts_10m_max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wind_gusts10m_max: Option<f64>,
    /// Daily mean wind speed at 10 meters (km/h)
    #[serde(rename = "wind_speed_10m_mean")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wind_speed10m_mean: Option<f64>,
    /// Minimum wind speed at 10 meters (km/h)
    #[serde(rename = "wind_speed_10m_min")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wind_speed10m_min: Option<f64>,
    /// Minimum wind gusts at 10 meters (km/h)
    #[serde(rename = "wind_gusts_10m_min")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wind_gusts10m_min: Option<f64>,
    /// Daily mean wind gusts at 10 meters (km/h)
    #[serde(rename = "wind_gusts_10m_mean")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wind_gusts10m_mean: Option<f64>,
    /// Dominant wind direction at 10 meters (°)
    #[serde(rename = "wind_direction_10m_dominant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_direction10m_dominant: Option<i64>,
    /// Daily sum of shortwave solar radiation (MJ/m²)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub shortwave_radiation_sum: Option<f64>,
    /// Daily sum of reference evapotranspiration (mm)
    #[serde(rename = "et0_fao_evapotranspiration_sum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub et0fao_evapotranspiration_sum: Option<f64>,
    /// Daily mean cloud cover percentage (%)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cloud_cover_mean: Option<f64>,
    /// Daily mean dew point temperature at 2 meters (°C)
    #[serde(rename = "dew_point_2m_mean")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub dew_point2m_mean: Option<f64>,
    /// Daily maximum dew point temperature at 2 meters (°C)
    #[serde(rename = "dew_point_2m_max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub dew_point2m_max: Option<f64>,
    /// Daily minimum dew point temperature at 2 meters (°C)
    #[serde(rename = "dew_point_2m_min")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub dew_point2m_min: Option<f64>,
    /// Daily mean relative humidity at 2 meters (%)
    #[serde(rename = "relative_humidity_2m_mean")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub relative_humidity2m_mean: Option<f64>,
    /// Daily maximum relative humidity at 2 meters (%)
    #[serde(rename = "relative_humidity_2m_max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_humidity2m_max: Option<i64>,
    /// Daily minimum relative humidity at 2 meters (%)
    #[serde(rename = "relative_humidity_2m_min")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_humidity2m_min: Option<i64>,
    /// Daily mean atmospheric pressure at mean sea level (hPa)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub pressure_msl_mean: Option<f64>,
    /// Daily mean surface pressure at ground level (hPa)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub surface_pressure_mean: Option<f64>,
}

impl WeatherTimeSeriesResponseHistoricalValueDaily {
    pub fn builder() -> WeatherTimeSeriesResponseHistoricalValueDailyBuilder {
        <WeatherTimeSeriesResponseHistoricalValueDailyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WeatherTimeSeriesResponseHistoricalValueDailyBuilder {
    timestamp: Option<DateTime<FixedOffset>>,
    weather_code: Option<i64>,
    temperature2m_mean: Option<f64>,
    temperature2m_max: Option<f64>,
    temperature2m_min: Option<f64>,
    apparent_temperature_mean: Option<f64>,
    apparent_temperature_max: Option<f64>,
    apparent_temperature_min: Option<f64>,
    precipitation_sum: Option<f64>,
    rain_sum: Option<f64>,
    snowfall_sum: Option<f64>,
    wind_speed10m_max: Option<f64>,
    wind_gusts10m_max: Option<f64>,
    wind_speed10m_mean: Option<f64>,
    wind_speed10m_min: Option<f64>,
    wind_gusts10m_min: Option<f64>,
    wind_gusts10m_mean: Option<f64>,
    wind_direction10m_dominant: Option<i64>,
    shortwave_radiation_sum: Option<f64>,
    et0fao_evapotranspiration_sum: Option<f64>,
    cloud_cover_mean: Option<f64>,
    dew_point2m_mean: Option<f64>,
    dew_point2m_max: Option<f64>,
    dew_point2m_min: Option<f64>,
    relative_humidity2m_mean: Option<f64>,
    relative_humidity2m_max: Option<i64>,
    relative_humidity2m_min: Option<i64>,
    pressure_msl_mean: Option<f64>,
    surface_pressure_mean: Option<f64>,
}

impl WeatherTimeSeriesResponseHistoricalValueDailyBuilder {
    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn weather_code(mut self, value: i64) -> Self {
        self.weather_code = Some(value);
        self
    }

    pub fn temperature2m_mean(mut self, value: f64) -> Self {
        self.temperature2m_mean = Some(value);
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

    pub fn apparent_temperature_mean(mut self, value: f64) -> Self {
        self.apparent_temperature_mean = Some(value);
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

    pub fn precipitation_sum(mut self, value: f64) -> Self {
        self.precipitation_sum = Some(value);
        self
    }

    pub fn rain_sum(mut self, value: f64) -> Self {
        self.rain_sum = Some(value);
        self
    }

    pub fn snowfall_sum(mut self, value: f64) -> Self {
        self.snowfall_sum = Some(value);
        self
    }

    pub fn wind_speed10m_max(mut self, value: f64) -> Self {
        self.wind_speed10m_max = Some(value);
        self
    }

    pub fn wind_gusts10m_max(mut self, value: f64) -> Self {
        self.wind_gusts10m_max = Some(value);
        self
    }

    pub fn wind_speed10m_mean(mut self, value: f64) -> Self {
        self.wind_speed10m_mean = Some(value);
        self
    }

    pub fn wind_speed10m_min(mut self, value: f64) -> Self {
        self.wind_speed10m_min = Some(value);
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

    pub fn et0fao_evapotranspiration_sum(mut self, value: f64) -> Self {
        self.et0fao_evapotranspiration_sum = Some(value);
        self
    }

    pub fn cloud_cover_mean(mut self, value: f64) -> Self {
        self.cloud_cover_mean = Some(value);
        self
    }

    pub fn dew_point2m_mean(mut self, value: f64) -> Self {
        self.dew_point2m_mean = Some(value);
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

    pub fn relative_humidity2m_mean(mut self, value: f64) -> Self {
        self.relative_humidity2m_mean = Some(value);
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

    pub fn pressure_msl_mean(mut self, value: f64) -> Self {
        self.pressure_msl_mean = Some(value);
        self
    }

    pub fn surface_pressure_mean(mut self, value: f64) -> Self {
        self.surface_pressure_mean = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WeatherTimeSeriesResponseHistoricalValueDaily`].
    pub fn build(self) -> Result<WeatherTimeSeriesResponseHistoricalValueDaily, BuildError> {
        Ok(WeatherTimeSeriesResponseHistoricalValueDaily {
            timestamp: self.timestamp,
            weather_code: self.weather_code,
            temperature2m_mean: self.temperature2m_mean,
            temperature2m_max: self.temperature2m_max,
            temperature2m_min: self.temperature2m_min,
            apparent_temperature_mean: self.apparent_temperature_mean,
            apparent_temperature_max: self.apparent_temperature_max,
            apparent_temperature_min: self.apparent_temperature_min,
            precipitation_sum: self.precipitation_sum,
            rain_sum: self.rain_sum,
            snowfall_sum: self.snowfall_sum,
            wind_speed10m_max: self.wind_speed10m_max,
            wind_gusts10m_max: self.wind_gusts10m_max,
            wind_speed10m_mean: self.wind_speed10m_mean,
            wind_speed10m_min: self.wind_speed10m_min,
            wind_gusts10m_min: self.wind_gusts10m_min,
            wind_gusts10m_mean: self.wind_gusts10m_mean,
            wind_direction10m_dominant: self.wind_direction10m_dominant,
            shortwave_radiation_sum: self.shortwave_radiation_sum,
            et0fao_evapotranspiration_sum: self.et0fao_evapotranspiration_sum,
            cloud_cover_mean: self.cloud_cover_mean,
            dew_point2m_mean: self.dew_point2m_mean,
            dew_point2m_max: self.dew_point2m_max,
            dew_point2m_min: self.dew_point2m_min,
            relative_humidity2m_mean: self.relative_humidity2m_mean,
            relative_humidity2m_max: self.relative_humidity2m_max,
            relative_humidity2m_min: self.relative_humidity2m_min,
            pressure_msl_mean: self.pressure_msl_mean,
            surface_pressure_mean: self.surface_pressure_mean,
        })
    }
}
