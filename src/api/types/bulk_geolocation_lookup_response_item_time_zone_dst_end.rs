pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkGeolocationLookupResponseItemTimeZoneDstEnd {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utc_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlap: Option<bool>,
}

impl BulkGeolocationLookupResponseItemTimeZoneDstEnd {
    pub fn builder() -> BulkGeolocationLookupResponseItemTimeZoneDstEndBuilder {
        <BulkGeolocationLookupResponseItemTimeZoneDstEndBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupResponseItemTimeZoneDstEndBuilder {
    utc_time: Option<String>,
    duration: Option<String>,
    gap: Option<bool>,
    date_time_after: Option<String>,
    date_time_before: Option<String>,
    overlap: Option<bool>,
}

impl BulkGeolocationLookupResponseItemTimeZoneDstEndBuilder {
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

    /// Consumes the builder and constructs a [`BulkGeolocationLookupResponseItemTimeZoneDstEnd`].
    pub fn build(self) -> Result<BulkGeolocationLookupResponseItemTimeZoneDstEnd, BuildError> {
        Ok(BulkGeolocationLookupResponseItemTimeZoneDstEnd {
            utc_time: self.utc_time,
            duration: self.duration,
            gap: self.gap,
            date_time_after: self.date_time_after,
            date_time_before: self.date_time_before,
            overlap: self.overlap,
        })
    }
}
