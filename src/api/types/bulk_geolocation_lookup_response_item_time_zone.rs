pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkGeolocationLookupResponseItemTimeZone {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub offset: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub offset_with_dst: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub current_time_unix: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_tz_abbreviation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_tz_full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_tz_abbreviation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_tz_full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_dst: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub dst_savings: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_exists: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_tz_abbreviation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_tz_full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_start: Option<BulkGeolocationLookupResponseItemTimeZoneDstStart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_end: Option<BulkGeolocationLookupResponseItemTimeZoneDstEnd>,
}

impl BulkGeolocationLookupResponseItemTimeZone {
    pub fn builder() -> BulkGeolocationLookupResponseItemTimeZoneBuilder {
        <BulkGeolocationLookupResponseItemTimeZoneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupResponseItemTimeZoneBuilder {
    name: Option<String>,
    offset: Option<f64>,
    offset_with_dst: Option<f64>,
    current_time: Option<String>,
    current_time_unix: Option<f64>,
    current_tz_abbreviation: Option<String>,
    current_tz_full_name: Option<String>,
    standard_tz_abbreviation: Option<String>,
    standard_tz_full_name: Option<String>,
    is_dst: Option<bool>,
    dst_savings: Option<f64>,
    dst_exists: Option<bool>,
    dst_tz_abbreviation: Option<String>,
    dst_tz_full_name: Option<String>,
    dst_start: Option<BulkGeolocationLookupResponseItemTimeZoneDstStart>,
    dst_end: Option<BulkGeolocationLookupResponseItemTimeZoneDstEnd>,
}

impl BulkGeolocationLookupResponseItemTimeZoneBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn offset(mut self, value: f64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn offset_with_dst(mut self, value: f64) -> Self {
        self.offset_with_dst = Some(value);
        self
    }

    pub fn current_time(mut self, value: impl Into<String>) -> Self {
        self.current_time = Some(value.into());
        self
    }

    pub fn current_time_unix(mut self, value: f64) -> Self {
        self.current_time_unix = Some(value);
        self
    }

    pub fn current_tz_abbreviation(mut self, value: impl Into<String>) -> Self {
        self.current_tz_abbreviation = Some(value.into());
        self
    }

    pub fn current_tz_full_name(mut self, value: impl Into<String>) -> Self {
        self.current_tz_full_name = Some(value.into());
        self
    }

    pub fn standard_tz_abbreviation(mut self, value: impl Into<String>) -> Self {
        self.standard_tz_abbreviation = Some(value.into());
        self
    }

    pub fn standard_tz_full_name(mut self, value: impl Into<String>) -> Self {
        self.standard_tz_full_name = Some(value.into());
        self
    }

    pub fn is_dst(mut self, value: bool) -> Self {
        self.is_dst = Some(value);
        self
    }

    pub fn dst_savings(mut self, value: f64) -> Self {
        self.dst_savings = Some(value);
        self
    }

    pub fn dst_exists(mut self, value: bool) -> Self {
        self.dst_exists = Some(value);
        self
    }

    pub fn dst_tz_abbreviation(mut self, value: impl Into<String>) -> Self {
        self.dst_tz_abbreviation = Some(value.into());
        self
    }

    pub fn dst_tz_full_name(mut self, value: impl Into<String>) -> Self {
        self.dst_tz_full_name = Some(value.into());
        self
    }

    pub fn dst_start(mut self, value: BulkGeolocationLookupResponseItemTimeZoneDstStart) -> Self {
        self.dst_start = Some(value);
        self
    }

    pub fn dst_end(mut self, value: BulkGeolocationLookupResponseItemTimeZoneDstEnd) -> Self {
        self.dst_end = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkGeolocationLookupResponseItemTimeZone`].
    pub fn build(self) -> Result<BulkGeolocationLookupResponseItemTimeZone, BuildError> {
        Ok(BulkGeolocationLookupResponseItemTimeZone {
            name: self.name,
            offset: self.offset,
            offset_with_dst: self.offset_with_dst,
            current_time: self.current_time,
            current_time_unix: self.current_time_unix,
            current_tz_abbreviation: self.current_tz_abbreviation,
            current_tz_full_name: self.current_tz_full_name,
            standard_tz_abbreviation: self.standard_tz_abbreviation,
            standard_tz_full_name: self.standard_tz_full_name,
            is_dst: self.is_dst,
            dst_savings: self.dst_savings,
            dst_exists: self.dst_exists,
            dst_tz_abbreviation: self.dst_tz_abbreviation,
            dst_tz_full_name: self.dst_tz_full_name,
            dst_start: self.dst_start,
            dst_end: self.dst_end,
        })
    }
}
