pub use crate::prelude::*;

/// Morning astronomical data including twilight, blue hour, and golden hour times.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AstronomyLookupV2ResponseAstronomyMorning {
    /// The start time of astronomical twilight in the morning
    #[serde(default)]
    pub astronomical_twilight_begin: String,
    /// The end time of astronomical twilight in the morning
    #[serde(default)]
    pub astronomical_twilight_end: String,
    /// The start time of nautical twilight in the morning
    #[serde(default)]
    pub nautical_twilight_begin: String,
    /// The end time of nautical twilight in the morning
    #[serde(default)]
    pub nautical_twilight_end: String,
    /// The start time of civil twilight in the morning
    #[serde(default)]
    pub civil_twilight_begin: String,
    /// The end time of civil twilight in the morning
    #[serde(default)]
    pub civil_twilight_end: String,
    /// The beginning of the blue hour in the morning
    #[serde(default)]
    pub blue_hour_begin: String,
    /// The end of the blue hour in the morning
    #[serde(default)]
    pub blue_hour_end: String,
    /// The beginning of the golden hour in the morning
    #[serde(default)]
    pub golden_hour_begin: String,
    /// The end of the golden hour in the morning
    #[serde(default)]
    pub golden_hour_end: String,
}

impl AstronomyLookupV2ResponseAstronomyMorning {
    pub fn builder() -> AstronomyLookupV2ResponseAstronomyMorningBuilder {
        <AstronomyLookupV2ResponseAstronomyMorningBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstronomyLookupV2ResponseAstronomyMorningBuilder {
    astronomical_twilight_begin: Option<String>,
    astronomical_twilight_end: Option<String>,
    nautical_twilight_begin: Option<String>,
    nautical_twilight_end: Option<String>,
    civil_twilight_begin: Option<String>,
    civil_twilight_end: Option<String>,
    blue_hour_begin: Option<String>,
    blue_hour_end: Option<String>,
    golden_hour_begin: Option<String>,
    golden_hour_end: Option<String>,
}

impl AstronomyLookupV2ResponseAstronomyMorningBuilder {
    pub fn astronomical_twilight_begin(mut self, value: impl Into<String>) -> Self {
        self.astronomical_twilight_begin = Some(value.into());
        self
    }

    pub fn astronomical_twilight_end(mut self, value: impl Into<String>) -> Self {
        self.astronomical_twilight_end = Some(value.into());
        self
    }

    pub fn nautical_twilight_begin(mut self, value: impl Into<String>) -> Self {
        self.nautical_twilight_begin = Some(value.into());
        self
    }

    pub fn nautical_twilight_end(mut self, value: impl Into<String>) -> Self {
        self.nautical_twilight_end = Some(value.into());
        self
    }

    pub fn civil_twilight_begin(mut self, value: impl Into<String>) -> Self {
        self.civil_twilight_begin = Some(value.into());
        self
    }

    pub fn civil_twilight_end(mut self, value: impl Into<String>) -> Self {
        self.civil_twilight_end = Some(value.into());
        self
    }

    pub fn blue_hour_begin(mut self, value: impl Into<String>) -> Self {
        self.blue_hour_begin = Some(value.into());
        self
    }

    pub fn blue_hour_end(mut self, value: impl Into<String>) -> Self {
        self.blue_hour_end = Some(value.into());
        self
    }

    pub fn golden_hour_begin(mut self, value: impl Into<String>) -> Self {
        self.golden_hour_begin = Some(value.into());
        self
    }

    pub fn golden_hour_end(mut self, value: impl Into<String>) -> Self {
        self.golden_hour_end = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AstronomyLookupV2ResponseAstronomyMorning`].
    /// This method will fail if any of the following fields are not set:
    /// - [`astronomical_twilight_begin`](AstronomyLookupV2ResponseAstronomyMorningBuilder::astronomical_twilight_begin)
    /// - [`astronomical_twilight_end`](AstronomyLookupV2ResponseAstronomyMorningBuilder::astronomical_twilight_end)
    /// - [`nautical_twilight_begin`](AstronomyLookupV2ResponseAstronomyMorningBuilder::nautical_twilight_begin)
    /// - [`nautical_twilight_end`](AstronomyLookupV2ResponseAstronomyMorningBuilder::nautical_twilight_end)
    /// - [`civil_twilight_begin`](AstronomyLookupV2ResponseAstronomyMorningBuilder::civil_twilight_begin)
    /// - [`civil_twilight_end`](AstronomyLookupV2ResponseAstronomyMorningBuilder::civil_twilight_end)
    /// - [`blue_hour_begin`](AstronomyLookupV2ResponseAstronomyMorningBuilder::blue_hour_begin)
    /// - [`blue_hour_end`](AstronomyLookupV2ResponseAstronomyMorningBuilder::blue_hour_end)
    /// - [`golden_hour_begin`](AstronomyLookupV2ResponseAstronomyMorningBuilder::golden_hour_begin)
    /// - [`golden_hour_end`](AstronomyLookupV2ResponseAstronomyMorningBuilder::golden_hour_end)
    pub fn build(self) -> Result<AstronomyLookupV2ResponseAstronomyMorning, BuildError> {
        Ok(AstronomyLookupV2ResponseAstronomyMorning {
            astronomical_twilight_begin: self
                .astronomical_twilight_begin
                .ok_or_else(|| BuildError::missing_field("astronomical_twilight_begin"))?,
            astronomical_twilight_end: self
                .astronomical_twilight_end
                .ok_or_else(|| BuildError::missing_field("astronomical_twilight_end"))?,
            nautical_twilight_begin: self
                .nautical_twilight_begin
                .ok_or_else(|| BuildError::missing_field("nautical_twilight_begin"))?,
            nautical_twilight_end: self
                .nautical_twilight_end
                .ok_or_else(|| BuildError::missing_field("nautical_twilight_end"))?,
            civil_twilight_begin: self
                .civil_twilight_begin
                .ok_or_else(|| BuildError::missing_field("civil_twilight_begin"))?,
            civil_twilight_end: self
                .civil_twilight_end
                .ok_or_else(|| BuildError::missing_field("civil_twilight_end"))?,
            blue_hour_begin: self
                .blue_hour_begin
                .ok_or_else(|| BuildError::missing_field("blue_hour_begin"))?,
            blue_hour_end: self
                .blue_hour_end
                .ok_or_else(|| BuildError::missing_field("blue_hour_end"))?,
            golden_hour_begin: self
                .golden_hour_begin
                .ok_or_else(|| BuildError::missing_field("golden_hour_begin"))?,
            golden_hour_end: self
                .golden_hour_end
                .ok_or_else(|| BuildError::missing_field("golden_hour_end"))?,
        })
    }
}
