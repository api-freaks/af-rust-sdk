pub use crate::prelude::*;

/// DST transition details (used for both the DST start and DST end transitions).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkGeolocationLookupV2ResponseItemAbuseTimeZoneDstStart {
    /// DST transition moment in UTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utc_time: Option<String>,
    /// Clock change at the DST transition, in hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// true if local time jumps forward (some times do not exist).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<bool>,
    /// Local date/time immediately after the DST transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_after: Option<String>,
    /// Local date/time immediately before the DST transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_before: Option<String>,
    /// true if local times repeat around the DST transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlap: Option<bool>,
}

impl BulkGeolocationLookupV2ResponseItemAbuseTimeZoneDstStart {
    pub fn builder() -> BulkGeolocationLookupV2ResponseItemAbuseTimeZoneDstStartBuilder {
        <BulkGeolocationLookupV2ResponseItemAbuseTimeZoneDstStartBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupV2ResponseItemAbuseTimeZoneDstStartBuilder {
    utc_time: Option<String>,
    duration: Option<String>,
    gap: Option<bool>,
    date_time_after: Option<String>,
    date_time_before: Option<String>,
    overlap: Option<bool>,
}

impl BulkGeolocationLookupV2ResponseItemAbuseTimeZoneDstStartBuilder {
    pub fn utc_time(mut self, value: impl Into<String>) -> Self {
        self.utc_time = Some(value.into());
        self
    }

    pub fn duration(mut self, value: impl Into<String>) -> Self {
        self.duration = Some(value.into());
        self
    }

    pub fn gap(mut self, value: bool) -> Self {
        self.gap = Some(value);
        self
    }

    pub fn date_time_after(mut self, value: impl Into<String>) -> Self {
        self.date_time_after = Some(value.into());
        self
    }

    pub fn date_time_before(mut self, value: impl Into<String>) -> Self {
        self.date_time_before = Some(value.into());
        self
    }

    pub fn overlap(mut self, value: bool) -> Self {
        self.overlap = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkGeolocationLookupV2ResponseItemAbuseTimeZoneDstStart`].
    pub fn build(
        self,
    ) -> Result<BulkGeolocationLookupV2ResponseItemAbuseTimeZoneDstStart, BuildError> {
        Ok(BulkGeolocationLookupV2ResponseItemAbuseTimeZoneDstStart {
            utc_time: self.utc_time,
            duration: self.duration,
            gap: self.gap,
            date_time_after: self.date_time_after,
            date_time_before: self.date_time_before,
            overlap: self.overlap,
        })
    }
}
