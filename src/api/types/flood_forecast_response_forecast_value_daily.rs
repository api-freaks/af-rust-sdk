pub use crate::prelude::*;

/// Daily flood forecast data for the date.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FloodForecastResponseForecastValueDaily {
    /// ISO 8601 formatted timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub timestamp: Option<DateTime<FixedOffset>>,
    /// The observed river discharge value (m³/s)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub river_discharge: Option<f64>,
    /// The mean river discharge (m³/s)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub river_discharge_mean: Option<f64>,
    /// The median river discharge (m³/s)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub river_discharge_median: Option<f64>,
    /// The maximum river discharge (m³/s)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub river_discharge_max: Option<f64>,
    /// The minimum river discharge (m³/s)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub river_discharge_min: Option<f64>,
    /// The 25th percentile of river discharge (m³/s)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub river_discharge_p25: Option<f64>,
    /// The 75th percentile of river discharge (m³/s)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub river_discharge_p75: Option<f64>,
}

impl FloodForecastResponseForecastValueDaily {
    pub fn builder() -> FloodForecastResponseForecastValueDailyBuilder {
        <FloodForecastResponseForecastValueDailyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FloodForecastResponseForecastValueDailyBuilder {
    timestamp: Option<DateTime<FixedOffset>>,
    river_discharge: Option<f64>,
    river_discharge_mean: Option<f64>,
    river_discharge_median: Option<f64>,
    river_discharge_max: Option<f64>,
    river_discharge_min: Option<f64>,
    river_discharge_p25: Option<f64>,
    river_discharge_p75: Option<f64>,
}

impl FloodForecastResponseForecastValueDailyBuilder {
    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn river_discharge(mut self, value: f64) -> Self {
        self.river_discharge = Some(value);
        self
    }

    pub fn river_discharge_mean(mut self, value: f64) -> Self {
        self.river_discharge_mean = Some(value);
        self
    }

    pub fn river_discharge_median(mut self, value: f64) -> Self {
        self.river_discharge_median = Some(value);
        self
    }

    pub fn river_discharge_max(mut self, value: f64) -> Self {
        self.river_discharge_max = Some(value);
        self
    }

    pub fn river_discharge_min(mut self, value: f64) -> Self {
        self.river_discharge_min = Some(value);
        self
    }

    pub fn river_discharge_p25(mut self, value: f64) -> Self {
        self.river_discharge_p25 = Some(value);
        self
    }

    pub fn river_discharge_p75(mut self, value: f64) -> Self {
        self.river_discharge_p75 = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FloodForecastResponseForecastValueDaily`].
    pub fn build(self) -> Result<FloodForecastResponseForecastValueDaily, BuildError> {
        Ok(FloodForecastResponseForecastValueDaily {
            timestamp: self.timestamp,
            river_discharge: self.river_discharge,
            river_discharge_mean: self.river_discharge_mean,
            river_discharge_median: self.river_discharge_median,
            river_discharge_max: self.river_discharge_max,
            river_discharge_min: self.river_discharge_min,
            river_discharge_p25: self.river_discharge_p25,
            river_discharge_p75: self.river_discharge_p75,
        })
    }
}
