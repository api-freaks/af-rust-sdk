pub use crate::prelude::*;

/// UN/LOCODE location details, present when queried by LO code.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimezoneLookupV2ResponseLoCodeDetails {
    /// A unique identifier for the location, often used in logistics and shipping (e.g., USNYC).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lo_code: Option<String>,
    /// The name of the city or location associated with the LO code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The code for the state, province or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_code: Option<String>,
    /// The ISO 3166-1 alpha-2 country code (e.g., US).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// The name of the country in an administrative context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>,
    /// The type of location as comma-separated list of facilities (e.g., Port, Rail Terminal, Road Terminal, Airport).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_type: Option<String>,
    /// The latitude coordinate of the location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<String>,
    /// The longitude coordinate of the location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<String>,
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
    state_code: Option<String>,
    country_code: Option<String>,
    country_name: Option<String>,
    location_type: Option<String>,
    latitude: Option<String>,
    longitude: Option<String>,
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

    pub fn latitude(mut self, value: impl Into<String>) -> Self {
        self.latitude = Some(value.into());
        self
    }

    pub fn longitude(mut self, value: impl Into<String>) -> Self {
        self.longitude = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimezoneLookupV2ResponseLoCodeDetails`].
    pub fn build(self) -> Result<TimezoneLookupV2ResponseLoCodeDetails, BuildError> {
        Ok(TimezoneLookupV2ResponseLoCodeDetails {
            lo_code: self.lo_code,
            city: self.city,
            state_code: self.state_code,
            country_code: self.country_code,
            country_name: self.country_name,
            location_type: self.location_type,
            latitude: self.latitude,
            longitude: self.longitude,
        })
    }
}
