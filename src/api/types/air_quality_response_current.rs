pub use crate::prelude::*;

/// Current air quality data
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AirQualityResponseCurrent {
    /// ISO 8601 formatted timestamp (iso8601).
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    /// Consolidated European Air Quality Index representing the highest value among individual pollutant indices. Ranges: 0-20 (good), 20-40 (fair), 40-60 (moderate), 60-80 (poor), 80-100 (very poor), >100 (extremely poor).
    #[serde(default)]
    pub european_aqi: i64,
    /// Consolidated U.S. Air Quality Index representing the highest value among individual pollutant indices. Ranges: 0-50 (good), 51-100 (moderate), 101-150 (unhealthy for sensitive groups), 151-200 (unhealthy), 201-300 (very unhealthy), 301-500 (hazardous).
    #[serde(default)]
    pub us_aqi: i64,
    /// Particulate matter with diameter less than 10 micrometers (μg/m³) measured at 10 meters above ground.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub pm10: f64,
    /// Particulate matter with diameter less than 2.5 micrometers (μg/m³) measured at 10 meters above ground.
    #[serde(rename = "pm2_5")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub pm25: f64,
    /// Atmospheric carbon monoxide gas concentration (μg/m³) at 10 meters above ground.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub carbon_monoxide: f64,
    /// Atmospheric nitrogen dioxide gas concentration (μg/m³) at 10 meters above ground.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub nitrogen_dioxide: f64,
    /// Atmospheric sulphur dioxide gas concentration (μg/m³) at 10 meters above ground.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub sulphur_dioxide: f64,
    /// Atmospheric ozone gas concentration (μg/m³) at 10 meters above ground.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub ozone: f64,
    /// Saharan dust particle concentration (μg/m³) at 10 meters above ground.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub dust: f64,
    /// Ultraviolet radiation intensity index accounting for cloud coverage.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub uv_index: f64,
    /// Aerosol optical depth at 550 nm wavelength indicating atmospheric haze levels.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub aerosol_optical_depth: f64,
    /// Ultraviolet radiation intensity index assuming cloud-free conditions.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub uv_index_clear_sky: f64,
}

impl AirQualityResponseCurrent {
    pub fn builder() -> AirQualityResponseCurrentBuilder {
        <AirQualityResponseCurrentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AirQualityResponseCurrentBuilder {
    timestamp: Option<DateTime<FixedOffset>>,
    european_aqi: Option<i64>,
    us_aqi: Option<i64>,
    pm10: Option<f64>,
    pm25: Option<f64>,
    carbon_monoxide: Option<f64>,
    nitrogen_dioxide: Option<f64>,
    sulphur_dioxide: Option<f64>,
    ozone: Option<f64>,
    dust: Option<f64>,
    uv_index: Option<f64>,
    aerosol_optical_depth: Option<f64>,
    uv_index_clear_sky: Option<f64>,
}

impl AirQualityResponseCurrentBuilder {
    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn european_aqi(mut self, value: i64) -> Self {
        self.european_aqi = Some(value);
        self
    }

    pub fn us_aqi(mut self, value: i64) -> Self {
        self.us_aqi = Some(value);
        self
    }

    pub fn pm10(mut self, value: f64) -> Self {
        self.pm10 = Some(value);
        self
    }

    pub fn pm25(mut self, value: f64) -> Self {
        self.pm25 = Some(value);
        self
    }

    pub fn carbon_monoxide(mut self, value: f64) -> Self {
        self.carbon_monoxide = Some(value);
        self
    }

    pub fn nitrogen_dioxide(mut self, value: f64) -> Self {
        self.nitrogen_dioxide = Some(value);
        self
    }

    pub fn sulphur_dioxide(mut self, value: f64) -> Self {
        self.sulphur_dioxide = Some(value);
        self
    }

    pub fn ozone(mut self, value: f64) -> Self {
        self.ozone = Some(value);
        self
    }

    pub fn dust(mut self, value: f64) -> Self {
        self.dust = Some(value);
        self
    }

    pub fn uv_index(mut self, value: f64) -> Self {
        self.uv_index = Some(value);
        self
    }

    pub fn aerosol_optical_depth(mut self, value: f64) -> Self {
        self.aerosol_optical_depth = Some(value);
        self
    }

    pub fn uv_index_clear_sky(mut self, value: f64) -> Self {
        self.uv_index_clear_sky = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AirQualityResponseCurrent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`timestamp`](AirQualityResponseCurrentBuilder::timestamp)
    /// - [`european_aqi`](AirQualityResponseCurrentBuilder::european_aqi)
    /// - [`us_aqi`](AirQualityResponseCurrentBuilder::us_aqi)
    /// - [`pm10`](AirQualityResponseCurrentBuilder::pm10)
    /// - [`pm25`](AirQualityResponseCurrentBuilder::pm25)
    /// - [`carbon_monoxide`](AirQualityResponseCurrentBuilder::carbon_monoxide)
    /// - [`nitrogen_dioxide`](AirQualityResponseCurrentBuilder::nitrogen_dioxide)
    /// - [`sulphur_dioxide`](AirQualityResponseCurrentBuilder::sulphur_dioxide)
    /// - [`ozone`](AirQualityResponseCurrentBuilder::ozone)
    /// - [`dust`](AirQualityResponseCurrentBuilder::dust)
    /// - [`uv_index`](AirQualityResponseCurrentBuilder::uv_index)
    /// - [`aerosol_optical_depth`](AirQualityResponseCurrentBuilder::aerosol_optical_depth)
    /// - [`uv_index_clear_sky`](AirQualityResponseCurrentBuilder::uv_index_clear_sky)
    pub fn build(self) -> Result<AirQualityResponseCurrent, BuildError> {
        Ok(AirQualityResponseCurrent {
            timestamp: self
                .timestamp
                .ok_or_else(|| BuildError::missing_field("timestamp"))?,
            european_aqi: self
                .european_aqi
                .ok_or_else(|| BuildError::missing_field("european_aqi"))?,
            us_aqi: self
                .us_aqi
                .ok_or_else(|| BuildError::missing_field("us_aqi"))?,
            pm10: self.pm10.ok_or_else(|| BuildError::missing_field("pm10"))?,
            pm25: self.pm25.ok_or_else(|| BuildError::missing_field("pm25"))?,
            carbon_monoxide: self
                .carbon_monoxide
                .ok_or_else(|| BuildError::missing_field("carbon_monoxide"))?,
            nitrogen_dioxide: self
                .nitrogen_dioxide
                .ok_or_else(|| BuildError::missing_field("nitrogen_dioxide"))?,
            sulphur_dioxide: self
                .sulphur_dioxide
                .ok_or_else(|| BuildError::missing_field("sulphur_dioxide"))?,
            ozone: self
                .ozone
                .ok_or_else(|| BuildError::missing_field("ozone"))?,
            dust: self.dust.ok_or_else(|| BuildError::missing_field("dust"))?,
            uv_index: self
                .uv_index
                .ok_or_else(|| BuildError::missing_field("uv_index"))?,
            aerosol_optical_depth: self
                .aerosol_optical_depth
                .ok_or_else(|| BuildError::missing_field("aerosol_optical_depth"))?,
            uv_index_clear_sky: self
                .uv_index_clear_sky
                .ok_or_else(|| BuildError::missing_field("uv_index_clear_sky"))?,
        })
    }
}
