pub use crate::prelude::*;

/// Current weather data
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkCurrentWeatherResponseBulkItemCurrent {
    /// ISO 8601 formatted timestamp of the current weather observation.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    /// Current air temperature (°C) measured at 2 meters above ground.
    #[serde(rename = "temperature_2m")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub temperature2m: f64,
    /// Current relative humidity percentage at 2 meters above ground.
    #[serde(rename = "relative_humidity_2m")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub relative_humidity2m: f64,
    /// Current apparent temperature (°C) accounting for wind chill and humidity.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub apparent_temperature: f64,
    /// Current snowfall accumulation in centimeters.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub snowfall: f64,
    /// Current rainfall accumulation in millimeters.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub rain: f64,
    /// Current shower precipitation in millimeters.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub showers: f64,
    /// Total precipitation (mm) including rain, showers, and snowfall.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub precipitation: f64,
    /// WMO weather condition code representing current conditions. Supported codes: 0 clear sky; 1, 2, 3 varying cloud cover; 45, 48 fog; 51, 53, 55 drizzle; 56, 57 freezing drizzle; 61, 63, 65 rain; 66, 67 freezing rain; 71, 73, 75 snowfall; 77 snow grains; 80, 81, 82 rain showers; 85, 86 snow showers; 95 thunderstorm; 96, 99 thunderstorm with hail.
    #[serde(default)]
    pub weather_code: i64,
    /// Current percentage of sky covered by clouds.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub cloud_cover: f64,
    /// Current atmospheric pressure (hPa) adjusted to mean sea level.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub pressure_msl: f64,
    /// Current atmospheric pressure (hPa) at surface level.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub surface_pressure: f64,
    /// Current wind speed (km/h) at 10 meters above ground.
    #[serde(rename = "wind_speed_10m")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub wind_speed10m: f64,
    /// Current wind direction in degrees at 10 meters above ground.
    #[serde(rename = "wind_direction_10m")]
    #[serde(default)]
    pub wind_direction10m: i64,
    /// Current wind gust speed (km/h) at 10 meters above ground.
    #[serde(rename = "wind_gusts_10m")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub wind_gusts10m: f64,
    /// Astronomical information including sunrise, sunset, and moon phases for the current date.
    #[serde(default)]
    pub astronomy: BulkCurrentWeatherResponseBulkItemCurrentAstronomy,
    /// Air quality metrics including pollutant concentrations and AQI values.
    #[serde(default)]
    pub air_quality: BulkCurrentWeatherResponseBulkItemCurrentAirQuality,
}

impl BulkCurrentWeatherResponseBulkItemCurrent {
    pub fn builder() -> BulkCurrentWeatherResponseBulkItemCurrentBuilder {
        <BulkCurrentWeatherResponseBulkItemCurrentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkCurrentWeatherResponseBulkItemCurrentBuilder {
    timestamp: Option<DateTime<FixedOffset>>,
    temperature2m: Option<f64>,
    relative_humidity2m: Option<f64>,
    apparent_temperature: Option<f64>,
    snowfall: Option<f64>,
    rain: Option<f64>,
    showers: Option<f64>,
    precipitation: Option<f64>,
    weather_code: Option<i64>,
    cloud_cover: Option<f64>,
    pressure_msl: Option<f64>,
    surface_pressure: Option<f64>,
    wind_speed10m: Option<f64>,
    wind_direction10m: Option<i64>,
    wind_gusts10m: Option<f64>,
    astronomy: Option<BulkCurrentWeatherResponseBulkItemCurrentAstronomy>,
    air_quality: Option<BulkCurrentWeatherResponseBulkItemCurrentAirQuality>,
}

impl BulkCurrentWeatherResponseBulkItemCurrentBuilder {
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

    pub fn apparent_temperature(mut self, value: f64) -> Self {
        self.apparent_temperature = Some(value);
        self
    }

    pub fn snowfall(mut self, value: f64) -> Self {
        self.snowfall = Some(value);
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

    pub fn precipitation(mut self, value: f64) -> Self {
        self.precipitation = Some(value);
        self
    }

    pub fn weather_code(mut self, value: i64) -> Self {
        self.weather_code = Some(value);
        self
    }

    pub fn cloud_cover(mut self, value: f64) -> Self {
        self.cloud_cover = Some(value);
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

    pub fn astronomy(mut self, value: BulkCurrentWeatherResponseBulkItemCurrentAstronomy) -> Self {
        self.astronomy = Some(value);
        self
    }

    pub fn air_quality(
        mut self,
        value: BulkCurrentWeatherResponseBulkItemCurrentAirQuality,
    ) -> Self {
        self.air_quality = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkCurrentWeatherResponseBulkItemCurrent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`timestamp`](BulkCurrentWeatherResponseBulkItemCurrentBuilder::timestamp)
    /// - [`temperature2m`](BulkCurrentWeatherResponseBulkItemCurrentBuilder::temperature2m)
    /// - [`relative_humidity2m`](BulkCurrentWeatherResponseBulkItemCurrentBuilder::relative_humidity2m)
    /// - [`apparent_temperature`](BulkCurrentWeatherResponseBulkItemCurrentBuilder::apparent_temperature)
    /// - [`snowfall`](BulkCurrentWeatherResponseBulkItemCurrentBuilder::snowfall)
    /// - [`rain`](BulkCurrentWeatherResponseBulkItemCurrentBuilder::rain)
    /// - [`showers`](BulkCurrentWeatherResponseBulkItemCurrentBuilder::showers)
    /// - [`precipitation`](BulkCurrentWeatherResponseBulkItemCurrentBuilder::precipitation)
    /// - [`weather_code`](BulkCurrentWeatherResponseBulkItemCurrentBuilder::weather_code)
    /// - [`cloud_cover`](BulkCurrentWeatherResponseBulkItemCurrentBuilder::cloud_cover)
    /// - [`pressure_msl`](BulkCurrentWeatherResponseBulkItemCurrentBuilder::pressure_msl)
    /// - [`surface_pressure`](BulkCurrentWeatherResponseBulkItemCurrentBuilder::surface_pressure)
    /// - [`wind_speed10m`](BulkCurrentWeatherResponseBulkItemCurrentBuilder::wind_speed10m)
    /// - [`wind_direction10m`](BulkCurrentWeatherResponseBulkItemCurrentBuilder::wind_direction10m)
    /// - [`wind_gusts10m`](BulkCurrentWeatherResponseBulkItemCurrentBuilder::wind_gusts10m)
    /// - [`astronomy`](BulkCurrentWeatherResponseBulkItemCurrentBuilder::astronomy)
    /// - [`air_quality`](BulkCurrentWeatherResponseBulkItemCurrentBuilder::air_quality)
    pub fn build(self) -> Result<BulkCurrentWeatherResponseBulkItemCurrent, BuildError> {
        Ok(BulkCurrentWeatherResponseBulkItemCurrent {
            timestamp: self
                .timestamp
                .ok_or_else(|| BuildError::missing_field("timestamp"))?,
            temperature2m: self
                .temperature2m
                .ok_or_else(|| BuildError::missing_field("temperature2m"))?,
            relative_humidity2m: self
                .relative_humidity2m
                .ok_or_else(|| BuildError::missing_field("relative_humidity2m"))?,
            apparent_temperature: self
                .apparent_temperature
                .ok_or_else(|| BuildError::missing_field("apparent_temperature"))?,
            snowfall: self
                .snowfall
                .ok_or_else(|| BuildError::missing_field("snowfall"))?,
            rain: self.rain.ok_or_else(|| BuildError::missing_field("rain"))?,
            showers: self
                .showers
                .ok_or_else(|| BuildError::missing_field("showers"))?,
            precipitation: self
                .precipitation
                .ok_or_else(|| BuildError::missing_field("precipitation"))?,
            weather_code: self
                .weather_code
                .ok_or_else(|| BuildError::missing_field("weather_code"))?,
            cloud_cover: self
                .cloud_cover
                .ok_or_else(|| BuildError::missing_field("cloud_cover"))?,
            pressure_msl: self
                .pressure_msl
                .ok_or_else(|| BuildError::missing_field("pressure_msl"))?,
            surface_pressure: self
                .surface_pressure
                .ok_or_else(|| BuildError::missing_field("surface_pressure"))?,
            wind_speed10m: self
                .wind_speed10m
                .ok_or_else(|| BuildError::missing_field("wind_speed10m"))?,
            wind_direction10m: self
                .wind_direction10m
                .ok_or_else(|| BuildError::missing_field("wind_direction10m"))?,
            wind_gusts10m: self
                .wind_gusts10m
                .ok_or_else(|| BuildError::missing_field("wind_gusts10m"))?,
            astronomy: self
                .astronomy
                .ok_or_else(|| BuildError::missing_field("astronomy"))?,
            air_quality: self
                .air_quality
                .ok_or_else(|| BuildError::missing_field("air_quality"))?,
        })
    }
}
