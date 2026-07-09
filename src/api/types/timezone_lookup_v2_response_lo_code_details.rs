pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimezoneLookupV2ResponseLoCodeDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lo_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_type: Option<String>,
}

impl TimezoneLookupV2ResponseLoCodeDetails {
    pub fn builder() -> TimezoneLookupV2ResponseLoCodeDetailsBuilder {
        <TimezoneLookupV2ResponseLoCodeDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimezoneLookupV2ResponseLoCodeDetailsBuilder {
    lo_code: Option<String>,
    city: Option<String>,
    longitude: Option<f64>,
    latitude: Option<f64>,
    state_code: Option<String>,
    country_code: Option<String>,
    country_name: Option<String>,
    location_type: Option<String>,
}

impl TimezoneLookupV2ResponseLoCodeDetailsBuilder {
    pub fn lo_code(mut self, value: impl Into<String>) -> Self {
        self.lo_code = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn longitude(mut self, value: f64) -> Self {
        self.longitude = Some(value);
        self
    }

    pub fn latitude(mut self, value: f64) -> Self {
        self.latitude = Some(value);
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

    /// Consumes the builder and constructs a [`TimezoneLookupV2ResponseLoCodeDetails`].
    pub fn build(self) -> Result<TimezoneLookupV2ResponseLoCodeDetails, BuildError> {
        Ok(TimezoneLookupV2ResponseLoCodeDetails {
            lo_code: self.lo_code,
            city: self.city,
            longitude: self.longitude,
            latitude: self.latitude,
            state_code: self.state_code,
            country_code: self.country_code,
            country_name: self.country_name,
            location_type: self.location_type,
        })
    }
}
