pub use crate::prelude::*;

/// Timezone and date/time information for the location.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimezoneLookupV2ResponseTimeZone {
    /// The IANA timezone name/identifier for the location.
    #[serde(default)]
    pub name: String,
    /// The Standard time zone offset from UTC in hours.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub offset: f64,
    /// The time zone offset from UTC in hours, accounting for DST.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub offset_with_dst: f64,
    /// The current date and time with timezone offset in YYYY-MM-DD HH:mm:ss.SSS±ZZZZ format.
    #[serde(default)]
    pub current_time: String,
    /// The Unix timestamp representing the date and time in seconds.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub current_time_unix: f64,
    /// The current date in YYYY-MM-DD format.
    #[serde(default)]
    pub date: String,
    /// The current date and time in YYYY-MM-DD HH:mm:ss format.
    #[serde(default)]
    pub date_time: String,
    /// The current date and time in descriptive format EEEE, MMMM dd, yyyy HH:mm:ss.
    #[serde(default)]
    pub date_time_txt: String,
    /// The date and time with time zone information in EEE, dd MMM yyyy HH:mm:ss Z format.
    #[serde(default)]
    pub date_time_wti: String,
    /// The date and time with timezone offset in ISO 8601 format YYYY-MM-DDTHH:mm:ss±HHMM.
    #[serde(default)]
    pub date_time_ymd: String,
    /// The current time in 24-hour format HH:mm:ss.
    #[serde(rename = "time_24")]
    #[serde(default)]
    pub time24: String,
    /// The current time in 12-hour format with AM/PM notation.
    #[serde(rename = "time_12")]
    #[serde(default)]
    pub time12: String,
    /// The week number of the year (1-52).
    #[serde(default)]
    pub week: i64,
    /// The current month as a number (1-12).
    #[serde(default)]
    pub month: i64,
    /// The four-digit current year.
    #[serde(default)]
    pub year: i64,
    /// The two-digit abbreviation for the year.
    #[serde(default)]
    pub year_abbr: String,
    /// Abbreviation of the time zone currently in effect (standard or DST).
    #[serde(default)]
    pub current_tz_abbreviation: String,
    /// Full name of the time zone currently in effect.
    #[serde(default)]
    pub current_tz_full_name: String,
    /// Abbreviation of the standard (non-DST) time zone.
    #[serde(default)]
    pub standard_tz_abbreviation: String,
    /// Full name of the standard (non-DST) time zone.
    #[serde(default)]
    pub standard_tz_full_name: String,
    /// Is the time zone in daylight savings?
    #[serde(default)]
    pub is_dst: bool,
    /// Abbreviation of the DST time zone. Always present as a key; holds an empty string when dst_exists is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_tz_abbreviation: Option<String>,
    /// Full name of the DST time zone. Always present as a key; holds an empty string when dst_exists is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_tz_full_name: Option<String>,
    /// The amount of time added for daylight saving in hours.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub dst_savings: f64,
    /// Indicates whether DST is observed in the region.
    #[serde(default)]
    pub dst_exists: bool,
    /// DST transition details (used for both the DST start and DST end transitions). Always present as a key on the parent TimeZone object; returned as an empty object {} when dst_exists is false, so none of its properties are required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_start: Option<TimezoneLookupV2ResponseTimeZoneDstStart>,
    /// DST transition details (used for both the DST start and DST end transitions). Always present as a key on the parent TimeZone object; returned as an empty object {} when dst_exists is false, so none of its properties are required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_end: Option<TimezoneLookupV2ResponseTimeZoneDstEnd>,
}

impl TimezoneLookupV2ResponseTimeZone {
    pub fn builder() -> TimezoneLookupV2ResponseTimeZoneBuilder {
        <TimezoneLookupV2ResponseTimeZoneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimezoneLookupV2ResponseTimeZoneBuilder {
    name: Option<String>,
    offset: Option<f64>,
    offset_with_dst: Option<f64>,
    current_time: Option<String>,
    current_time_unix: Option<f64>,
    date: Option<String>,
    date_time: Option<String>,
    date_time_txt: Option<String>,
    date_time_wti: Option<String>,
    date_time_ymd: Option<String>,
    time24: Option<String>,
    time12: Option<String>,
    week: Option<i64>,
    month: Option<i64>,
    year: Option<i64>,
    year_abbr: Option<String>,
    current_tz_abbreviation: Option<String>,
    current_tz_full_name: Option<String>,
    standard_tz_abbreviation: Option<String>,
    standard_tz_full_name: Option<String>,
    is_dst: Option<bool>,
    dst_tz_abbreviation: Option<String>,
    dst_tz_full_name: Option<String>,
    dst_savings: Option<f64>,
    dst_exists: Option<bool>,
    dst_start: Option<TimezoneLookupV2ResponseTimeZoneDstStart>,
    dst_end: Option<TimezoneLookupV2ResponseTimeZoneDstEnd>,
}

impl TimezoneLookupV2ResponseTimeZoneBuilder {
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

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn date_time(mut self, value: impl Into<String>) -> Self {
        self.date_time = Some(value.into());
        self
    }

    pub fn date_time_txt(mut self, value: impl Into<String>) -> Self {
        self.date_time_txt = Some(value.into());
        self
    }

    pub fn date_time_wti(mut self, value: impl Into<String>) -> Self {
        self.date_time_wti = Some(value.into());
        self
    }

    pub fn date_time_ymd(mut self, value: impl Into<String>) -> Self {
        self.date_time_ymd = Some(value.into());
        self
    }

    pub fn time24(mut self, value: impl Into<String>) -> Self {
        self.time24 = Some(value.into());
        self
    }

    pub fn time12(mut self, value: impl Into<String>) -> Self {
        self.time12 = Some(value.into());
        self
    }

    pub fn week(mut self, value: i64) -> Self {
        self.week = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn year_abbr(mut self, value: impl Into<String>) -> Self {
        self.year_abbr = Some(value.into());
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

    pub fn dst_tz_abbreviation(mut self, value: impl Into<String>) -> Self {
        self.dst_tz_abbreviation = Some(value.into());
        self
    }

    pub fn dst_tz_full_name(mut self, value: impl Into<String>) -> Self {
        self.dst_tz_full_name = Some(value.into());
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

    pub fn dst_start(mut self, value: TimezoneLookupV2ResponseTimeZoneDstStart) -> Self {
        self.dst_start = Some(value);
        self
    }

    pub fn dst_end(mut self, value: TimezoneLookupV2ResponseTimeZoneDstEnd) -> Self {
        self.dst_end = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimezoneLookupV2ResponseTimeZone`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](TimezoneLookupV2ResponseTimeZoneBuilder::name)
    /// - [`offset`](TimezoneLookupV2ResponseTimeZoneBuilder::offset)
    /// - [`offset_with_dst`](TimezoneLookupV2ResponseTimeZoneBuilder::offset_with_dst)
    /// - [`current_time`](TimezoneLookupV2ResponseTimeZoneBuilder::current_time)
    /// - [`current_time_unix`](TimezoneLookupV2ResponseTimeZoneBuilder::current_time_unix)
    /// - [`date`](TimezoneLookupV2ResponseTimeZoneBuilder::date)
    /// - [`date_time`](TimezoneLookupV2ResponseTimeZoneBuilder::date_time)
    /// - [`date_time_txt`](TimezoneLookupV2ResponseTimeZoneBuilder::date_time_txt)
    /// - [`date_time_wti`](TimezoneLookupV2ResponseTimeZoneBuilder::date_time_wti)
    /// - [`date_time_ymd`](TimezoneLookupV2ResponseTimeZoneBuilder::date_time_ymd)
    /// - [`time24`](TimezoneLookupV2ResponseTimeZoneBuilder::time24)
    /// - [`time12`](TimezoneLookupV2ResponseTimeZoneBuilder::time12)
    /// - [`week`](TimezoneLookupV2ResponseTimeZoneBuilder::week)
    /// - [`month`](TimezoneLookupV2ResponseTimeZoneBuilder::month)
    /// - [`year`](TimezoneLookupV2ResponseTimeZoneBuilder::year)
    /// - [`year_abbr`](TimezoneLookupV2ResponseTimeZoneBuilder::year_abbr)
    /// - [`current_tz_abbreviation`](TimezoneLookupV2ResponseTimeZoneBuilder::current_tz_abbreviation)
    /// - [`current_tz_full_name`](TimezoneLookupV2ResponseTimeZoneBuilder::current_tz_full_name)
    /// - [`standard_tz_abbreviation`](TimezoneLookupV2ResponseTimeZoneBuilder::standard_tz_abbreviation)
    /// - [`standard_tz_full_name`](TimezoneLookupV2ResponseTimeZoneBuilder::standard_tz_full_name)
    /// - [`is_dst`](TimezoneLookupV2ResponseTimeZoneBuilder::is_dst)
    /// - [`dst_savings`](TimezoneLookupV2ResponseTimeZoneBuilder::dst_savings)
    /// - [`dst_exists`](TimezoneLookupV2ResponseTimeZoneBuilder::dst_exists)
    pub fn build(self) -> Result<TimezoneLookupV2ResponseTimeZone, BuildError> {
        Ok(TimezoneLookupV2ResponseTimeZone {
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
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            date_time: self
                .date_time
                .ok_or_else(|| BuildError::missing_field("date_time"))?,
            date_time_txt: self
                .date_time_txt
                .ok_or_else(|| BuildError::missing_field("date_time_txt"))?,
            date_time_wti: self
                .date_time_wti
                .ok_or_else(|| BuildError::missing_field("date_time_wti"))?,
            date_time_ymd: self
                .date_time_ymd
                .ok_or_else(|| BuildError::missing_field("date_time_ymd"))?,
            time24: self
                .time24
                .ok_or_else(|| BuildError::missing_field("time24"))?,
            time12: self
                .time12
                .ok_or_else(|| BuildError::missing_field("time12"))?,
            week: self.week.ok_or_else(|| BuildError::missing_field("week"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            year_abbr: self
                .year_abbr
                .ok_or_else(|| BuildError::missing_field("year_abbr"))?,
            current_tz_abbreviation: self
                .current_tz_abbreviation
                .ok_or_else(|| BuildError::missing_field("current_tz_abbreviation"))?,
            current_tz_full_name: self
                .current_tz_full_name
                .ok_or_else(|| BuildError::missing_field("current_tz_full_name"))?,
            standard_tz_abbreviation: self
                .standard_tz_abbreviation
                .ok_or_else(|| BuildError::missing_field("standard_tz_abbreviation"))?,
            standard_tz_full_name: self
                .standard_tz_full_name
                .ok_or_else(|| BuildError::missing_field("standard_tz_full_name"))?,
            is_dst: self
                .is_dst
                .ok_or_else(|| BuildError::missing_field("is_dst"))?,
            dst_tz_abbreviation: self.dst_tz_abbreviation,
            dst_tz_full_name: self.dst_tz_full_name,
            dst_savings: self
                .dst_savings
                .ok_or_else(|| BuildError::missing_field("dst_savings"))?,
            dst_exists: self
                .dst_exists
                .ok_or_else(|| BuildError::missing_field("dst_exists"))?,
            dst_start: self.dst_start,
            dst_end: self.dst_end,
        })
    }
}
