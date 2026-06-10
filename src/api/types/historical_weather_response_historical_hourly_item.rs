pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct HistoricalWeatherResponseHistoricalHourlyItem {
    /// ISO 8601 formatted timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub timestamp: Option<DateTime<FixedOffset>>,
    /// Air temperature at 2 meters (°C)
    #[serde(rename = "temperature_2m")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub temperature2m: Option<f64>,
    /// Relative humidity at 2 meters (%)
    #[serde(rename = "relative_humidity_2m")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub relative_humidity2m: Option<f64>,
    /// Dew point temperature at 2 meters (°C)
    #[serde(rename = "dew_point_2m")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub dew_point2m: Option<f64>,
    /// Perceived temperature (°C)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub apparent_temperature: Option<f64>,
    /// Total precipitation in this hour (mm)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub precipitation: Option<f64>,
    /// Rainfall in this hour (mm)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub rain: Option<f64>,
    /// Snowfall in this hour (cm)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub snowfall: Option<f64>,
    /// Weather condition code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weather_code: Option<i64>,
    /// Atmospheric pressure at mean sea level (hPa)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub pressure_msl: Option<f64>,
    /// Atmospheric pressure at ground level (hPa)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub surface_pressure: Option<f64>,
    /// Cloud cover percentage (%)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cloud_cover: Option<f64>,
    /// Hourly reference evapotranspiration (mm)
    #[serde(rename = "et0_fao_evapotranspiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub et0fao_evapotranspiration: Option<f64>,
    /// Wind speed at 10 meters (km/h)
    #[serde(rename = "wind_speed_10m")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wind_speed10m: Option<f64>,
    /// Wind direction at 10 meters (°)
    #[serde(rename = "wind_direction_10m")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_direction10m: Option<i64>,
    /// Wind gusts at 10 meters (km/h)
    #[serde(rename = "wind_gusts_10m")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wind_gusts10m: Option<f64>,
    /// Reflectivity of the Earth's surface
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub albedo: Option<f64>,
    /// Incoming shortwave solar radiation (W/m²)
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
    /// Direct solar irradiance (W/m²)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub direct_normal_irradiance: Option<f64>,
    /// Total solar irradiance on a tilted surface (W/m²)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub global_tilted_irradiance: Option<f64>,
}

impl HistoricalWeatherResponseHistoricalHourlyItem {
    pub fn builder() -> HistoricalWeatherResponseHistoricalHourlyItemBuilder {
        <HistoricalWeatherResponseHistoricalHourlyItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HistoricalWeatherResponseHistoricalHourlyItemBuilder {
    timestamp: Option<DateTime<FixedOffset>>,
    temperature2m: Option<f64>,
    relative_humidity2m: Option<f64>,
    dew_point2m: Option<f64>,
    apparent_temperature: Option<f64>,
    precipitation: Option<f64>,
    rain: Option<f64>,
    snowfall: Option<f64>,
    weather_code: Option<i64>,
    pressure_msl: Option<f64>,
    surface_pressure: Option<f64>,
    cloud_cover: Option<f64>,
    et0fao_evapotranspiration: Option<f64>,
    wind_speed10m: Option<f64>,
    wind_direction10m: Option<i64>,
    wind_gusts10m: Option<f64>,
    albedo: Option<f64>,
    shortwave_radiation: Option<f64>,
    direct_radiation: Option<f64>,
    diffuse_radiation: Option<f64>,
    direct_normal_irradiance: Option<f64>,
    global_tilted_irradiance: Option<f64>,
}

impl HistoricalWeatherResponseHistoricalHourlyItemBuilder {
    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn temperature2m(mut self, value: f64) -> Self {
        self.temperature2m = Some(value);
        self
    }

    pub fn relative_humidity2m(mut self, value: f64) -> Self {
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

    pub fn rain(mut self, value: f64) -> Self {
        self.rain = Some(value);
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

    pub fn cloud_cover(mut self, value: f64) -> Self {
        self.cloud_cover = Some(value);
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

    pub fn albedo(mut self, value: f64) -> Self {
        self.albedo = Some(value);
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

    /// Consumes the builder and constructs a [`HistoricalWeatherResponseHistoricalHourlyItem`].
    pub fn build(self) -> Result<HistoricalWeatherResponseHistoricalHourlyItem, BuildError> {
        Ok(HistoricalWeatherResponseHistoricalHourlyItem {
            timestamp: self.timestamp,
            temperature2m: self.temperature2m,
            relative_humidity2m: self.relative_humidity2m,
            dew_point2m: self.dew_point2m,
            apparent_temperature: self.apparent_temperature,
            precipitation: self.precipitation,
            rain: self.rain,
            snowfall: self.snowfall,
            weather_code: self.weather_code,
            pressure_msl: self.pressure_msl,
            surface_pressure: self.surface_pressure,
            cloud_cover: self.cloud_cover,
            et0fao_evapotranspiration: self.et0fao_evapotranspiration,
            wind_speed10m: self.wind_speed10m,
            wind_direction10m: self.wind_direction10m,
            wind_gusts10m: self.wind_gusts10m,
            albedo: self.albedo,
            shortwave_radiation: self.shortwave_radiation,
            direct_radiation: self.direct_radiation,
            diffuse_radiation: self.diffuse_radiation,
            direct_normal_irradiance: self.direct_normal_irradiance,
            global_tilted_irradiance: self.global_tilted_irradiance,
        })
    }
}
