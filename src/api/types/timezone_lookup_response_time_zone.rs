pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimezoneLookupResponseTimeZone {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub offset: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub offset_with_dst: f64,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub date_time: String,
    #[serde(default)]
    pub date_time_txt: String,
    #[serde(default)]
    pub date_time_wti: String,
    #[serde(default)]
    pub date_time_ymd: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub date_time_unix: f64,
    #[serde(rename = "time_24")]
    #[serde(default)]
    pub time24: String,
    #[serde(rename = "time_12")]
    #[serde(default)]
    pub time12: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub week: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub month: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub year: f64,
    #[serde(default)]
    pub year_abbr: String,
    #[serde(default)]
    pub current_tz_abbreviation: String,
    #[serde(default)]
    pub current_tz_full_name: String,
    #[serde(default)]
    pub standard_tz_abbreviation: String,
    #[serde(default)]
    pub standard_tz_full_name: String,
    #[serde(default)]
    pub is_dst: bool,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub dst_savings: f64,
    #[serde(default)]
    pub dst_exists: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_start: Option<TimezoneLookupResponseTimeZoneDstStart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_end: Option<TimezoneLookupResponseTimeZoneDstEnd>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl TimezoneLookupResponseTimeZone {
    pub fn builder() -> TimezoneLookupResponseTimeZoneBuilder {
        <TimezoneLookupResponseTimeZoneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimezoneLookupResponseTimeZoneBuilder {
    name: Option<String>,
    offset: Option<f64>,
    offset_with_dst: Option<f64>,
    date: Option<String>,
    date_time: Option<String>,
    date_time_txt: Option<String>,
    date_time_wti: Option<String>,
    date_time_ymd: Option<String>,
    date_time_unix: Option<f64>,
    time24: Option<String>,
    time12: Option<String>,
    week: Option<f64>,
    month: Option<f64>,
    year: Option<f64>,
    year_abbr: Option<String>,
    current_tz_abbreviation: Option<String>,
    current_tz_full_name: Option<String>,
    standard_tz_abbreviation: Option<String>,
    standard_tz_full_name: Option<String>,
    is_dst: Option<bool>,
    dst_savings: Option<f64>,
    dst_exists: Option<bool>,
    dst_start: Option<TimezoneLookupResponseTimeZoneDstStart>,
    dst_end: Option<TimezoneLookupResponseTimeZoneDstEnd>,
}

impl TimezoneLookupResponseTimeZoneBuilder {
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

    pub fn date_time_unix(mut self, value: f64) -> Self {
        self.date_time_unix = Some(value);
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

    pub fn week(mut self, value: f64) -> Self {
        self.week = Some(value);
        self
    }

    pub fn month(mut self, value: f64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn year(mut self, value: f64) -> Self {
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

    pub fn dst_savings(mut self, value: f64) -> Self {
        self.dst_savings = Some(value);
        self
    }

    pub fn dst_exists(mut self, value: bool) -> Self {
        self.dst_exists = Some(value);
        self
    }

    pub fn dst_start(mut self, value: TimezoneLookupResponseTimeZoneDstStart) -> Self {
        self.dst_start = Some(value);
        self
    }

    pub fn dst_end(mut self, value: TimezoneLookupResponseTimeZoneDstEnd) -> Self {
        self.dst_end = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimezoneLookupResponseTimeZone`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](TimezoneLookupResponseTimeZoneBuilder::name)
    /// - [`offset`](TimezoneLookupResponseTimeZoneBuilder::offset)
    /// - [`offset_with_dst`](TimezoneLookupResponseTimeZoneBuilder::offset_with_dst)
    /// - [`date`](TimezoneLookupResponseTimeZoneBuilder::date)
    /// - [`date_time`](TimezoneLookupResponseTimeZoneBuilder::date_time)
    /// - [`date_time_txt`](TimezoneLookupResponseTimeZoneBuilder::date_time_txt)
    /// - [`date_time_wti`](TimezoneLookupResponseTimeZoneBuilder::date_time_wti)
    /// - [`date_time_ymd`](TimezoneLookupResponseTimeZoneBuilder::date_time_ymd)
    /// - [`date_time_unix`](TimezoneLookupResponseTimeZoneBuilder::date_time_unix)
    /// - [`time24`](TimezoneLookupResponseTimeZoneBuilder::time24)
    /// - [`time12`](TimezoneLookupResponseTimeZoneBuilder::time12)
    /// - [`week`](TimezoneLookupResponseTimeZoneBuilder::week)
    /// - [`month`](TimezoneLookupResponseTimeZoneBuilder::month)
    /// - [`year`](TimezoneLookupResponseTimeZoneBuilder::year)
    /// - [`year_abbr`](TimezoneLookupResponseTimeZoneBuilder::year_abbr)
    /// - [`current_tz_abbreviation`](TimezoneLookupResponseTimeZoneBuilder::current_tz_abbreviation)
    /// - [`current_tz_full_name`](TimezoneLookupResponseTimeZoneBuilder::current_tz_full_name)
    /// - [`standard_tz_abbreviation`](TimezoneLookupResponseTimeZoneBuilder::standard_tz_abbreviation)
    /// - [`standard_tz_full_name`](TimezoneLookupResponseTimeZoneBuilder::standard_tz_full_name)
    /// - [`is_dst`](TimezoneLookupResponseTimeZoneBuilder::is_dst)
    /// - [`dst_savings`](TimezoneLookupResponseTimeZoneBuilder::dst_savings)
    /// - [`dst_exists`](TimezoneLookupResponseTimeZoneBuilder::dst_exists)
    pub fn build(self) -> Result<TimezoneLookupResponseTimeZone, BuildError> {
        Ok(TimezoneLookupResponseTimeZone {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            offset: self
                .offset
                .ok_or_else(|| BuildError::missing_field("offset"))?,
            offset_with_dst: self
                .offset_with_dst
                .ok_or_else(|| BuildError::missing_field("offset_with_dst"))?,
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
            date_time_unix: self
                .date_time_unix
                .ok_or_else(|| BuildError::missing_field("date_time_unix"))?,
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
            dst_savings: self
                .dst_savings
                .ok_or_else(|| BuildError::missing_field("dst_savings"))?,
            dst_exists: self
                .dst_exists
                .ok_or_else(|| BuildError::missing_field("dst_exists"))?,
            dst_start: self.dst_start,
            dst_end: self.dst_end,
            extra: Default::default(),
        })
    }
}
