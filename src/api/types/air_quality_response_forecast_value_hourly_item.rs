pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AirQualityResponseForecastValueHourlyItem {
    /// ISO 8601 formatted timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub timestamp: Option<DateTime<FixedOffset>>,
    /// Concentration of particulate matter ≤10 micrometers (μg/m³)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub pm10: Option<f64>,
    /// Concentration of carbon monoxide (μg/m³)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub carbon_monoxide: Option<f64>,
    /// Concentration of particulate matter ≤2.5 micrometers (μg/m³)
    #[serde(rename = "pm2_5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub pm25: Option<f64>,
    /// Concentration of carbon dioxide (ppm)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub carbon_dioxide: Option<f64>,
    /// Concentration of nitrogen dioxide (μg/m³)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub nitrogen_dioxide: Option<f64>,
    /// Concentration of sulphur dioxide (μg/m³)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub sulphur_dioxide: Option<f64>,
    /// Concentration of ozone (μg/m³)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub ozone: Option<f64>,
    /// Concentration of dust particles (μg/m³)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub dust: Option<f64>,
    /// Ultraviolet radiation index
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub uv_index: Option<f64>,
    /// Aerosol optical depth
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub aerosol_optical_depth: Option<f64>,
    /// Ultraviolet radiation index under clear sky conditions
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub uv_index_clear_sky: Option<f64>,
}

impl AirQualityResponseForecastValueHourlyItem {
    pub fn builder() -> AirQualityResponseForecastValueHourlyItemBuilder {
        <AirQualityResponseForecastValueHourlyItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AirQualityResponseForecastValueHourlyItemBuilder {
    timestamp: Option<DateTime<FixedOffset>>,
    pm10: Option<f64>,
    carbon_monoxide: Option<f64>,
    pm25: Option<f64>,
    carbon_dioxide: Option<f64>,
    nitrogen_dioxide: Option<f64>,
    sulphur_dioxide: Option<f64>,
    ozone: Option<f64>,
    dust: Option<f64>,
    uv_index: Option<f64>,
    aerosol_optical_depth: Option<f64>,
    uv_index_clear_sky: Option<f64>,
}

impl AirQualityResponseForecastValueHourlyItemBuilder {
    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn pm10(mut self, value: f64) -> Self {
        self.pm10 = Some(value);
        self
    }

    pub fn carbon_monoxide(mut self, value: f64) -> Self {
        self.carbon_monoxide = Some(value);
        self
    }

    pub fn pm25(mut self, value: f64) -> Self {
        self.pm25 = Some(value);
        self
    }

    pub fn carbon_dioxide(mut self, value: f64) -> Self {
        self.carbon_dioxide = Some(value);
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

    /// Consumes the builder and constructs a [`AirQualityResponseForecastValueHourlyItem`].
    pub fn build(self) -> Result<AirQualityResponseForecastValueHourlyItem, BuildError> {
        Ok(AirQualityResponseForecastValueHourlyItem {
            timestamp: self.timestamp,
            pm10: self.pm10,
            carbon_monoxide: self.carbon_monoxide,
            pm25: self.pm25,
            carbon_dioxide: self.carbon_dioxide,
            nitrogen_dioxide: self.nitrogen_dioxide,
            sulphur_dioxide: self.sulphur_dioxide,
            ozone: self.ozone,
            dust: self.dust,
            uv_index: self.uv_index,
            aerosol_optical_depth: self.aerosol_optical_depth,
            uv_index_clear_sky: self.uv_index_clear_sky,
        })
    }
}
