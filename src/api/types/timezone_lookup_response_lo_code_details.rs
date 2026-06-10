pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimezoneLookupResponseLoCodeDetails {
    #[serde(default)]
    pub lo_code: String,
    #[serde(default)]
    pub city: String,
    #[serde(default)]
    pub longitude: String,
    #[serde(default)]
    pub latitude: String,
    #[serde(default)]
    pub state_code: String,
    #[serde(default)]
    pub country_code: String,
    #[serde(default)]
    pub country_name: String,
    #[serde(default)]
    pub location_type: String,
}

impl TimezoneLookupResponseLoCodeDetails {
    pub fn builder() -> TimezoneLookupResponseLoCodeDetailsBuilder {
        <TimezoneLookupResponseLoCodeDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimezoneLookupResponseLoCodeDetailsBuilder {
    lo_code: Option<String>,
    city: Option<String>,
    longitude: Option<String>,
    latitude: Option<String>,
    state_code: Option<String>,
    country_code: Option<String>,
    country_name: Option<String>,
    location_type: Option<String>,
}

impl TimezoneLookupResponseLoCodeDetailsBuilder {
    pub fn lo_code(mut self, value: impl Into<String>) -> Self {
        self.lo_code = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn longitude(mut self, value: impl Into<String>) -> Self {
        self.longitude = Some(value.into());
        self
    }

    pub fn latitude(mut self, value: impl Into<String>) -> Self {
        self.latitude = Some(value.into());
        self
    }

    pub fn state_code(mut self, value: impl Into<String>) -> Self {
        self.state_code = Some(value.into());
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn country_name(mut self, value: impl Into<String>) -> Self {
        self.country_name = Some(value.into());
        self
    }

    pub fn location_type(mut self, value: impl Into<String>) -> Self {
        self.location_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimezoneLookupResponseLoCodeDetails`].
    /// This method will fail if any of the following fields are not set:
    /// - [`lo_code`](TimezoneLookupResponseLoCodeDetailsBuilder::lo_code)
    /// - [`city`](TimezoneLookupResponseLoCodeDetailsBuilder::city)
    /// - [`longitude`](TimezoneLookupResponseLoCodeDetailsBuilder::longitude)
    /// - [`latitude`](TimezoneLookupResponseLoCodeDetailsBuilder::latitude)
    /// - [`state_code`](TimezoneLookupResponseLoCodeDetailsBuilder::state_code)
    /// - [`country_code`](TimezoneLookupResponseLoCodeDetailsBuilder::country_code)
    /// - [`country_name`](TimezoneLookupResponseLoCodeDetailsBuilder::country_name)
    /// - [`location_type`](TimezoneLookupResponseLoCodeDetailsBuilder::location_type)
    pub fn build(self) -> Result<TimezoneLookupResponseLoCodeDetails, BuildError> {
        Ok(TimezoneLookupResponseLoCodeDetails {
            lo_code: self
                .lo_code
                .ok_or_else(|| BuildError::missing_field("lo_code"))?,
            city: self.city.ok_or_else(|| BuildError::missing_field("city"))?,
            longitude: self
                .longitude
                .ok_or_else(|| BuildError::missing_field("longitude"))?,
            latitude: self
                .latitude
                .ok_or_else(|| BuildError::missing_field("latitude"))?,
            state_code: self
                .state_code
                .ok_or_else(|| BuildError::missing_field("state_code"))?,
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            country_name: self
                .country_name
                .ok_or_else(|| BuildError::missing_field("country_name"))?,
            location_type: self
                .location_type
                .ok_or_else(|| BuildError::missing_field("location_type"))?,
        })
    }
}
