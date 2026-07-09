pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AstronomyLookupV2ResponseLocation {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name_official: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_eu: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_prov: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zipcode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevation: Option<String>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl AstronomyLookupV2ResponseLocation {
    pub fn builder() -> AstronomyLookupV2ResponseLocationBuilder {
        <AstronomyLookupV2ResponseLocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstronomyLookupV2ResponseLocationBuilder {
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

impl AstronomyLookupV2ResponseLocationBuilder {
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

    /// Consumes the builder and constructs a [`AstronomyLookupV2ResponseLocation`].
    pub fn build(self) -> Result<AstronomyLookupV2ResponseLocation, BuildError> {
        Ok(AstronomyLookupV2ResponseLocation {
            location_string: self.location_string,
            continent_code: self.continent_code,
            continent_name: self.continent_name,
            country_code2: self.country_code2,
            country_code3: self.country_code3,
            country_name: self.country_name,
            country_name_official: self.country_name_official,
            is_eu: self.is_eu,
            state_prov: self.state_prov,
            state_code: self.state_code,
            district: self.district,
            city: self.city,
            zipcode: self.zipcode,
            latitude: self.latitude,
            longitude: self.longitude,
            locality: self.locality,
            elevation: self.elevation,
            extra: Default::default(),
        })
    }
}
