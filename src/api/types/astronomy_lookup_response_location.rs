pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AstronomyLookupResponseLocation {
    /// The complete geographical location (returned when looking up by the location parameter).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code3: Option<String>,
    #[serde(default)]
    pub country_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name_official: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_eu: Option<bool>,
    #[serde(default)]
    pub state_prov: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    #[serde(default)]
    pub city: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zipcode: Option<String>,
    #[serde(default)]
    pub latitude: String,
    #[serde(default)]
    pub longitude: String,
    #[serde(default)]
    pub locality: String,
    #[serde(default)]
    pub elevation: String,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl AstronomyLookupResponseLocation {
    pub fn builder() -> AstronomyLookupResponseLocationBuilder {
        <AstronomyLookupResponseLocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstronomyLookupResponseLocationBuilder {
    location_string: Option<String>,
    continent_code: Option<String>,
    continent_name: Option<String>,
    country_code2: Option<String>,
    country_code3: Option<String>,
    country_name: Option<String>,
    country_name_official: Option<String>,
    is_eu: Option<bool>,
    state_prov: Option<String>,
    state_code: Option<String>,
    district: Option<String>,
    city: Option<String>,
    zipcode: Option<String>,
    latitude: Option<String>,
    longitude: Option<String>,
    locality: Option<String>,
    elevation: Option<String>,
}

impl AstronomyLookupResponseLocationBuilder {
    pub fn location_string(mut self, value: impl Into<String>) -> Self {
        self.location_string = Some(value.into());
        self
    }

    pub fn continent_code(mut self, value: impl Into<String>) -> Self {
        self.continent_code = Some(value.into());
        self
    }

    pub fn continent_name(mut self, value: impl Into<String>) -> Self {
        self.continent_name = Some(value.into());
        self
    }

    pub fn country_code2(mut self, value: impl Into<String>) -> Self {
        self.country_code2 = Some(value.into());
        self
    }

    pub fn country_code3(mut self, value: impl Into<String>) -> Self {
        self.country_code3 = Some(value.into());
        self
    }

    pub fn country_name(mut self, value: impl Into<String>) -> Self {
        self.country_name = Some(value.into());
        self
    }

    pub fn country_name_official(mut self, value: impl Into<String>) -> Self {
        self.country_name_official = Some(value.into());
        self
    }

    pub fn is_eu(mut self, value: bool) -> Self {
        self.is_eu = Some(value);
        self
    }

    pub fn state_prov(mut self, value: impl Into<String>) -> Self {
        self.state_prov = Some(value.into());
        self
    }

    pub fn state_code(mut self, value: impl Into<String>) -> Self {
        self.state_code = Some(value.into());
        self
    }

    pub fn district(mut self, value: impl Into<String>) -> Self {
        self.district = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn zipcode(mut self, value: impl Into<String>) -> Self {
        self.zipcode = Some(value.into());
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

    pub fn locality(mut self, value: impl Into<String>) -> Self {
        self.locality = Some(value.into());
        self
    }

    pub fn elevation(mut self, value: impl Into<String>) -> Self {
        self.elevation = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AstronomyLookupResponseLocation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_name`](AstronomyLookupResponseLocationBuilder::country_name)
    /// - [`state_prov`](AstronomyLookupResponseLocationBuilder::state_prov)
    /// - [`city`](AstronomyLookupResponseLocationBuilder::city)
    /// - [`latitude`](AstronomyLookupResponseLocationBuilder::latitude)
    /// - [`longitude`](AstronomyLookupResponseLocationBuilder::longitude)
    /// - [`locality`](AstronomyLookupResponseLocationBuilder::locality)
    /// - [`elevation`](AstronomyLookupResponseLocationBuilder::elevation)
    pub fn build(self) -> Result<AstronomyLookupResponseLocation, BuildError> {
        Ok(AstronomyLookupResponseLocation {
            location_string: self.location_string,
            continent_code: self.continent_code,
            continent_name: self.continent_name,
            country_code2: self.country_code2,
            country_code3: self.country_code3,
            country_name: self
                .country_name
                .ok_or_else(|| BuildError::missing_field("country_name"))?,
            country_name_official: self.country_name_official,
            is_eu: self.is_eu,
            state_prov: self
                .state_prov
                .ok_or_else(|| BuildError::missing_field("state_prov"))?,
            state_code: self.state_code,
            district: self.district,
            city: self.city.ok_or_else(|| BuildError::missing_field("city"))?,
            zipcode: self.zipcode,
            latitude: self
                .latitude
                .ok_or_else(|| BuildError::missing_field("latitude"))?,
            longitude: self
                .longitude
                .ok_or_else(|| BuildError::missing_field("longitude"))?,
            locality: self
                .locality
                .ok_or_else(|| BuildError::missing_field("locality"))?,
            elevation: self
                .elevation
                .ok_or_else(|| BuildError::missing_field("elevation"))?,
            extra: Default::default(),
        })
    }
}
