pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WeatherForecastResponseForecastValueHourlyItem {
    /// ISO 8601 formatted timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub timestamp: Option<DateTime<FixedOffset>>,
    /// Air temperature at 2m (°C)
    #[serde(rename = "temperature_2m")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub temperature2m: Option<f64>,
    /// Relative humidity at 2m (%)
    #[serde(rename = "relative_humidity_2m")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_humidity2m: Option<i64>,
    /// Dew point at 2m (°C)
    #[serde(rename = "dew_point_2m")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub dew_point2m: Option<f64>,
    /// Feels-like temperature (°C)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub apparent_temperature: Option<f64>,
    /// Total precipitation at this time (mm)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub precipitation: Option<f64>,
    /// Probability of precipitation (%)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precipitation_probability: Option<i64>,
    /// Rainfall (mm)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub rain: Option<f64>,
    /// Showers (mm)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub showers: Option<f64>,
    /// Snowfall (cm)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub snowfall: Option<f64>,
    /// Weather condition code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weather_code: Option<i64>,
    /// Sea-level pressure (hPa)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub pressure_msl: Option<f64>,
    /// Surface pressure (hPa)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub surface_pressure: Option<f64>,
    /// Cloud cover (%)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_cover: Option<i64>,
    /// Visibility distance (m)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub visibility: Option<f64>,
    /// Evapotranspiration (mm)
    #[serde(rename = "et0_fao_evapotranspiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub et0fao_evapotranspiration: Option<f64>,
    /// Wind speed at 10m (km/h)
    #[serde(rename = "wind_speed_10m")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wind_speed10m: Option<f64>,
    /// Wind direction at 10m (°)
    #[serde(rename = "wind_direction_10m")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_direction10m: Option<i64>,
    /// Wind gusts at 10m (km/h)
    #[serde(rename = "wind_gusts_10m")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wind_gusts10m: Option<f64>,
    /// UV index
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub uv_index: Option<f64>,
    /// UV index under clear-sky conditions
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub uv_index_clear_sky: Option<f64>,
    /// Shortwave radiation (W/m²)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub shortwave_radiation: Option<f64>,
    /// Direct solar radiation (W/m²)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub direct_radiation: Option<f64>,
    /// Diffuse solar radiation (W/m²)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub diffuse_radiation: Option<f64>,
    /// Direct normal irradiance (W/m²)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub direct_normal_irradiance: Option<f64>,
    /// Global tilted irradiance (W/m²)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub global_tilted_irradiance: Option<f64>,
}

impl WeatherForecastResponseForecastValueHourlyItem {
    pub fn builder() -> WeatherForecastResponseForecastValueHourlyItemBuilder {
        <WeatherForecastResponseForecastValueHourlyItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WeatherForecastResponseForecastValueHourlyItemBuilder {
    timestamp: Option<DateTime<FixedOffset>>,
    temperature2m: Option<f64>,
    relative_humidity2m: Option<i64>,
    dew_point2m: Option<f64>,
    apparent_temperature: Option<f64>,
    precipitation: Option<f64>,
    precipitation_probability: Option<i64>,
    rain: Option<f64>,
    showers: Option<f64>,
    snowfall: Option<f64>,
    weather_code: Option<i64>,
    pressure_msl: Option<f64>,
    surface_pressure: Option<f64>,
    cloud_cover: Option<i64>,
    visibility: Option<f64>,
    et0fao_evapotranspiration: Option<f64>,
    wind_speed10m: Option<f64>,
    wind_direction10m: Option<i64>,
    wind_gusts10m: Option<f64>,
    uv_index: Option<f64>,
    uv_index_clear_sky: Option<f64>,
    shortwave_radiation: Option<f64>,
    direct_radiation: Option<f64>,
    diffuse_radiation: Option<f64>,
    direct_normal_irradiance: Option<f64>,
    global_tilted_irradiance: Option<f64>,
}

impl WeatherForecastResponseForecastValueHourlyItemBuilder {
    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn temperature2m(mut self, value: f64) -> Self {
        self.temperature2m = Some(value);
        self
    }

    pub fn relative_humidity2m(mut self, value: i64) -> Self {
        self.relative_humidity2m = Some(value);
        self
    }

    pub fn dew_point2m(mut self, value: f64) -> Self {
        self.dew_point2m = Some(value);
        self
    }

    pub fn apparent_temperature(mut self, value: f64) -> Self {
        self.apparent_temperature = Some(value);
        self
    }

    pub fn precipitation(mut self, value: f64) -> Self {
        self.precipitation = Some(value);
        self
    }

    pub fn precipitation_probability(mut self, value: i64) -> Self {
        self.precipitation_probability = Some(value);
        self
    }

    pub fn rain(mut self, value: f64) -> Self {
        self.rain = Some(value);
        self
    }

    pub fn showers(mut self, value: f64) -> Self {
        self.showers = Some(value);
        self
    }

    pub fn snowfall(mut self, value: f64) -> Self {
        self.snowfall = Some(value);
        self
    }

    pub fn weather_code(mut self, value: i64) -> Self {
        self.weather_code = Some(value);
        self
    }

    pub fn pressure_msl(mut self, value: f64) -> Self {
        self.pressure_msl = Some(value);
        self
    }

    pub fn surface_pressure(mut self, value: f64) -> Self {
        self.surface_pressure = Some(value);
        self
    }

    pub fn cloud_cover(mut self, value: i64) -> Self {
        self.cloud_cover = Some(value);
        self
    }

    pub fn visibility(mut self, value: f64) -> Self {
        self.visibility = Some(value);
        self
    }

    pub fn et0fao_evapotranspiration(mut self, value: f64) -> Self {
        self.et0fao_evapotranspiration = Some(value);
        self
    }

    pub fn wind_speed10m(mut self, value: f64) -> Self {
        self.wind_speed10m = Some(value);
        self
    }

    pub fn wind_direction10m(mut self, value: i64) -> Self {
        self.wind_direction10m = Some(value);
        self
    }

    pub fn wind_gusts10m(mut self, value: f64) -> Self {
        self.wind_gusts10m = Some(value);
        self
    }

    pub fn uv_index(mut self, value: f64) -> Self {
        self.uv_index = Some(value);
        self
    }

    pub fn uv_index_clear_sky(mut self, value: f64) -> Self {
        self.uv_index_clear_sky = Some(value);
        self
    }

    pub fn shortwave_radiation(mut self, value: f64) -> Self {
        self.shortwave_radiation = Some(value);
        self
    }

    pub fn direct_radiation(mut self, value: f64) -> Self {
        self.direct_radiation = Some(value);
        self
    }

    pub fn diffuse_radiation(mut self, value: f64) -> Self {
        self.diffuse_radiation = Some(value);
        self
    }

    pub fn direct_normal_irradiance(mut self, value: f64) -> Self {
        self.direct_normal_irradiance = Some(value);
        self
    }

    pub fn global_tilted_irradiance(mut self, value: f64) -> Self {
        self.global_tilted_irradiance = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WeatherForecastResponseForecastValueHourlyItem`].
    pub fn build(self) -> Result<WeatherForecastResponseForecastValueHourlyItem, BuildError> {
        Ok(WeatherForecastResponseForecastValueHourlyItem {
            timestamp: self.timestamp,
            temperature2m: self.temperature2m,
            relative_humidity2m: self.relative_humidity2m,
            dew_point2m: self.dew_point2m,
            apparent_temperature: self.apparent_temperature,
            precipitation: self.precipitation,
            precipitation_probability: self.precipitation_probability,
            rain: self.rain,
            showers: self.showers,
            snowfall: self.snowfall,
            weather_code: self.weather_code,
            pressure_msl: self.pressure_msl,
            surface_pressure: self.surface_pressure,
            cloud_cover: self.cloud_cover,
            visibility: self.visibility,
            et0fao_evapotranspiration: self.et0fao_evapotranspiration,
            wind_speed10m: self.wind_speed10m,
            wind_direction10m: self.wind_direction10m,
            wind_gusts10m: self.wind_gusts10m,
            uv_index: self.uv_index,
            uv_index_clear_sky: self.uv_index_clear_sky,
            shortwave_radiation: self.shortwave_radiation,
            direct_radiation: self.direct_radiation,
            diffuse_radiation: self.diffuse_radiation,
            direct_normal_irradiance: self.direct_normal_irradiance,
            global_tilted_irradiance: self.global_tilted_irradiance,
        })
    }
}
