pub use crate::prelude::*;

/// Time zone information for the IP's location.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GeolocationLookupV2ResponseTimeZone {
    /// Time zone in IANA TZDB format.
    #[serde(default)]
    pub name: String,
    /// Standard time UTC offset in hours.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub offset: f64,
    /// Current effective UTC offset in hours, including DST.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub offset_with_dst: f64,
    /// Local date/time in YYYY-MM-DD HH:mm:ss.SSS±ZZZZ format.
    #[serde(default)]
    pub current_time: String,
    /// Local time as Unix epoch seconds.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub current_time_unix: f64,
    /// Current time zone abbreviation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_tz_abbreviation: Option<String>,
    /// Current time zone full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_tz_full_name: Option<String>,
    /// Standard (non-DST) abbreviation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_tz_abbreviation: Option<String>,
    /// Standard (non-DST) full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_tz_full_name: Option<String>,
    /// true if DST is active.
    #[serde(default)]
    pub is_dst: bool,
    /// DST shift amount in hours.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub dst_savings: f64,
    /// true if the time zone observes DST.
    #[serde(default)]
    pub dst_exists: bool,
    /// DST abbreviation when DST is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_tz_abbreviation: Option<String>,
    /// DST full name when DST is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_tz_full_name: Option<String>,
    /// DST transition details (used for both the DST start and DST end transitions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_start: Option<GeolocationLookupV2ResponseTimeZoneDstStart>,
    /// DST transition details (used for both the DST start and DST end transitions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_end: Option<GeolocationLookupV2ResponseTimeZoneDstEnd>,
}

impl GeolocationLookupV2ResponseTimeZone {
    pub fn builder() -> GeolocationLookupV2ResponseTimeZoneBuilder {
        <GeolocationLookupV2ResponseTimeZoneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeolocationLookupV2ResponseTimeZoneBuilder {
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
    dst_start: Option<GeolocationLookupV2ResponseTimeZoneDstStart>,
    dst_end: Option<GeolocationLookupV2ResponseTimeZoneDstEnd>,
}

impl GeolocationLookupV2ResponseTimeZoneBuilder {
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

    pub fn dst_start(mut self, value: GeolocationLookupV2ResponseTimeZoneDstStart) -> Self {
        self.dst_start = Some(value);
        self
    }

    pub fn dst_end(mut self, value: GeolocationLookupV2ResponseTimeZoneDstEnd) -> Self {
        self.dst_end = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeolocationLookupV2ResponseTimeZone`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](GeolocationLookupV2ResponseTimeZoneBuilder::name)
    /// - [`offset`](GeolocationLookupV2ResponseTimeZoneBuilder::offset)
    /// - [`offset_with_dst`](GeolocationLookupV2ResponseTimeZoneBuilder::offset_with_dst)
    /// - [`current_time`](GeolocationLookupV2ResponseTimeZoneBuilder::current_time)
    /// - [`current_time_unix`](GeolocationLookupV2ResponseTimeZoneBuilder::current_time_unix)
    /// - [`is_dst`](GeolocationLookupV2ResponseTimeZoneBuilder::is_dst)
    /// - [`dst_savings`](GeolocationLookupV2ResponseTimeZoneBuilder::dst_savings)
    /// - [`dst_exists`](GeolocationLookupV2ResponseTimeZoneBuilder::dst_exists)
    pub fn build(self) -> Result<GeolocationLookupV2ResponseTimeZone, BuildError> {
        Ok(GeolocationLookupV2ResponseTimeZone {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            offset: self
                .offset
                .ok_or_else(|| BuildError::missing_field("offset"))?,
            offset_with_dst: self
                .offset_with_dst
                .ok_or_else(|| BuildError::missing_field("offset_with_dst"))?,
            current_time: self
                .current_time
                .ok_or_else(|| BuildError::missing_field("current_time"))?,
            current_time_unix: self
                .current_time_unix
                .ok_or_else(|| BuildError::missing_field("current_time_unix"))?,
            current_tz_abbreviation: self.current_tz_abbreviation,
            current_tz_full_name: self.current_tz_full_name,
            standard_tz_abbreviation: self.standard_tz_abbreviation,
            standard_tz_full_name: self.standard_tz_full_name,
            is_dst: self
                .is_dst
                .ok_or_else(|| BuildError::missing_field("is_dst"))?,
            dst_savings: self
                .dst_savings
                .ok_or_else(|| BuildError::missing_field("dst_savings"))?,
            dst_exists: self
                .dst_exists
                .ok_or_else(|| BuildError::missing_field("dst_exists"))?,
            dst_tz_abbreviation: self.dst_tz_abbreviation,
            dst_tz_full_name: self.dst_tz_full_name,
            dst_start: self.dst_start,
            dst_end: self.dst_end,
        })
    }
}
