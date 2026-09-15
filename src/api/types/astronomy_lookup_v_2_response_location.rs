pub use crate::prelude::*;

/// Geographic location information for the astronomy calculation. The set of populated fields depends on which lookup mode the request used: (1) location param (geocode-by-address) returns location_string plus a basic field set (country_name, state_prov, city, locality, latitude, longitude, elevation); (2) lat + long params (geocode-by-coordinates) returns the same basic field set minus location_string, and locality may be an empty string when the coordinates don't resolve to a named sub-area; (3) ip param, or no location/lat/long/ip param at all (falls back to geo-IP lookup of the client's IP), returns the full geo-IP field set — continent_code, continent_name, country_code2, country_code3, country_name_official, is_eu, state_code, district, zipcode — in addition to the basic fields, but never location_string. elevation can be an empty string when elevation data is unavailable for the resolved location.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AstronomyLookupV2ResponseLocation {
    /// The location query parameter echoed back as-is. Present only for geocode-by-address lookups (location param); absent for lat/long and ip-based lookups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_string: Option<String>,
    /// The two-letter code of the continent (e.g., "NA"). Geo-IP field only: present for ip param or default client-IP lookups; absent for location/lat/long geocode lookups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent_code: Option<String>,
    /// The full name of the continent (e.g., "North America"). Geo-IP field only: present for ip param or default client-IP lookups; absent for location/lat/long geocode lookups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent_name: Option<String>,
    /// The ISO 3166-1 alpha-2 two-letter country code (e.g., "US"). Geo-IP field only: present for ip param or default client-IP lookups; absent for location/lat/long geocode lookups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code2: Option<String>,
    /// The ISO 3166-1 alpha-3 three-letter country code (e.g., "USA"). Geo-IP field only: present for ip param or default client-IP lookups; absent for location/lat/long geocode lookups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code3: Option<String>,
    /// The common name of the country (e.g., "United States"). Present in all lookup modes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>,
    /// The official full name of the country (e.g., "United States of America"). Geo-IP field only: present for ip param or default client-IP lookups; absent for location/lat/long geocode lookups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name_official: Option<String>,
    /// Whether the country belongs to the European Union. Geo-IP field only: present for ip param or default client-IP lookups; absent for location/lat/long geocode lookups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_eu: Option<bool>,
    /// Name of the state/province/region. Present in all lookup modes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_prov: Option<String>,
    /// Code of the state/province/region. Geo-IP field only: present for ip param or default client-IP lookups; absent for location/lat/long geocode lookups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_code: Option<String>,
    /// Name of the district or county. Geo-IP field only: present for ip param or default client-IP lookups; absent for location/lat/long geocode lookups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    /// Name of the city. Present in all lookup modes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// ZIP/Postal code of the place. Geo-IP field only: present for ip param or default client-IP lookups; absent for location/lat/long geocode lookups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zipcode: Option<String>,
    /// The geographic latitude of the location. Present in all lookup modes.
    #[serde(default)]
    pub latitude: String,
    /// The geographic longitude of the location. Present in all lookup modes.
    #[serde(default)]
    pub longitude: String,
    /// Smaller area, part or region of a city. Present in all lookup modes, but may be an empty string for lat/long or geo-IP lookups when no named sub-area resolves.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// The elevation of the geographical location, in meters. Present in all lookup modes, but may be an empty string when elevation data is unavailable for the resolved location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevation: Option<String>,
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
    /// This method will fail if any of the following fields are not set:
    /// - [`latitude`](AstronomyLookupV2ResponseLocationBuilder::latitude)
    /// - [`longitude`](AstronomyLookupV2ResponseLocationBuilder::longitude)
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
            latitude: self
                .latitude
                .ok_or_else(|| BuildError::missing_field("latitude"))?,
            longitude: self
                .longitude
                .ok_or_else(|| BuildError::missing_field("longitude"))?,
            locality: self.locality,
            elevation: self.elevation,
        })
    }
}
