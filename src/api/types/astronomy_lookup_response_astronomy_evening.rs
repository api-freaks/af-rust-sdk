pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AstronomyLookupResponseAstronomyEvening {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub golden_hour_begin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub golden_hour_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_hour_begin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_hour_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub civil_twilight_begin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub civil_twilight_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nautical_twilight_begin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nautical_twilight_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub astronomical_twilight_begin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub astronomical_twilight_end: Option<String>,
}

impl AstronomyLookupResponseAstronomyEvening {
    pub fn builder() -> AstronomyLookupResponseAstronomyEveningBuilder {
        <AstronomyLookupResponseAstronomyEveningBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstronomyLookupResponseAstronomyEveningBuilder {
    golden_hour_begin: Option<String>,
    golden_hour_end: Option<String>,
    blue_hour_begin: Option<String>,
    blue_hour_end: Option<String>,
    civil_twilight_begin: Option<String>,
    civil_twilight_end: Option<String>,
    nautical_twilight_begin: Option<String>,
    nautical_twilight_end: Option<String>,
    astronomical_twilight_begin: Option<String>,
    astronomical_twilight_end: Option<String>,
}

impl AstronomyLookupResponseAstronomyEveningBuilder {
    pub fn golden_hour_begin(mut self, value: impl Into<String>) -> Self {
        self.golden_hour_begin = Some(value.into());
        self
    }

    pub fn golden_hour_end(mut self, value: impl Into<String>) -> Self {
        self.golden_hour_end = Some(value.into());
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

    pub fn civil_twilight_begin(mut self, value: impl Into<String>) -> Self {
        self.civil_twilight_begin = Some(value.into());
        self
    }

    pub fn civil_twilight_end(mut self, value: impl Into<String>) -> Self {
        self.civil_twilight_end = Some(value.into());
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

    pub fn astronomical_twilight_begin(mut self, value: impl Into<String>) -> Self {
        self.astronomical_twilight_begin = Some(value.into());
        self
    }

    pub fn astronomical_twilight_end(mut self, value: impl Into<String>) -> Self {
        self.astronomical_twilight_end = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AstronomyLookupResponseAstronomyEvening`].
    pub fn build(self) -> Result<AstronomyLookupResponseAstronomyEvening, BuildError> {
        Ok(AstronomyLookupResponseAstronomyEvening {
            golden_hour_begin: self.golden_hour_begin,
            golden_hour_end: self.golden_hour_end,
            blue_hour_begin: self.blue_hour_begin,
            blue_hour_end: self.blue_hour_end,
            civil_twilight_begin: self.civil_twilight_begin,
            civil_twilight_end: self.civil_twilight_end,
            nautical_twilight_begin: self.nautical_twilight_begin,
            nautical_twilight_end: self.nautical_twilight_end,
            astronomical_twilight_begin: self.astronomical_twilight_begin,
            astronomical_twilight_end: self.astronomical_twilight_end,
        })
    }
}
