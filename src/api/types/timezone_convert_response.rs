pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimezoneConvertResponse {
    /// Original time before conversion
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub original_time: DateTime<FixedOffset>,
    /// Time after conversion
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub converted_time: DateTime<FixedOffset>,
    /// Difference in hours
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub diff_hour: f64,
    /// Difference in minutes
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub diff_min: f64,
}

impl TimezoneConvertResponse {
    pub fn builder() -> TimezoneConvertResponseBuilder {
        <TimezoneConvertResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimezoneConvertResponseBuilder {
    original_time: Option<DateTime<FixedOffset>>,
    converted_time: Option<DateTime<FixedOffset>>,
    diff_hour: Option<f64>,
    diff_min: Option<f64>,
}

impl TimezoneConvertResponseBuilder {
    pub fn original_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.original_time = Some(value);
        self
    }

    pub fn converted_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.converted_time = Some(value);
        self
    }

    pub fn diff_hour(mut self, value: f64) -> Self {
        self.diff_hour = Some(value);
        self
    }

    pub fn diff_min(mut self, value: f64) -> Self {
        self.diff_min = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimezoneConvertResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`original_time`](TimezoneConvertResponseBuilder::original_time)
    /// - [`converted_time`](TimezoneConvertResponseBuilder::converted_time)
    /// - [`diff_hour`](TimezoneConvertResponseBuilder::diff_hour)
    /// - [`diff_min`](TimezoneConvertResponseBuilder::diff_min)
    pub fn build(self) -> Result<TimezoneConvertResponse, BuildError> {
        Ok(TimezoneConvertResponse {
            original_time: self
                .original_time
                .ok_or_else(|| BuildError::missing_field("original_time"))?,
            converted_time: self
                .converted_time
                .ok_or_else(|| BuildError::missing_field("converted_time"))?,
            diff_hour: self
                .diff_hour
                .ok_or_else(|| BuildError::missing_field("diff_hour"))?,
            diff_min: self
                .diff_min
                .ok_or_else(|| BuildError::missing_field("diff_min"))?,
        })
    }
}
