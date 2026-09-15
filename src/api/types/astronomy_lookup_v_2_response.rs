pub use crate::prelude::*;

/// Astronomy data response containing location information and astronomical data.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AstronomyLookupV2Response {
    /// IPv4 or IPv6 address used for the geo-IP lookup. Present when the ip parameter is passed explicitly, or when no location, lat/long, or ip parameter is supplied at all (the API falls back to the requesting client's IP address). Absent when location or lat/long is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Geographic location information for the astronomy calculation. The set of populated fields depends on which lookup mode the request used: (1) location param (geocode-by-address) returns location_string plus a basic field set (country_name, state_prov, city, locality, latitude, longitude, elevation); (2) lat + long params (geocode-by-coordinates) returns the same basic field set minus location_string, and locality may be an empty string when the coordinates don't resolve to a named sub-area; (3) ip param, or no location/lat/long/ip param at all (falls back to geo-IP lookup of the client's IP), returns the full geo-IP field set — continent_code, continent_name, country_code2, country_code3, country_name_official, is_eu, state_code, district, zipcode — in addition to the basic fields, but never location_string. elevation can be an empty string when elevation data is unavailable for the resolved location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<AstronomyLookupV2ResponseLocation>,
    /// Complete astronomical data for the specified location and date.
    #[serde(default)]
    pub astronomy: AstronomyLookupV2ResponseAstronomy,
}

impl AstronomyLookupV2Response {
    pub fn builder() -> AstronomyLookupV2ResponseBuilder {
        <AstronomyLookupV2ResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstronomyLookupV2ResponseBuilder {
    ip: Option<String>,
    location: Option<AstronomyLookupV2ResponseLocation>,
    astronomy: Option<AstronomyLookupV2ResponseAstronomy>,
}

impl AstronomyLookupV2ResponseBuilder {
    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn location(mut self, value: AstronomyLookupV2ResponseLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn astronomy(mut self, value: AstronomyLookupV2ResponseAstronomy) -> Self {
        self.astronomy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstronomyLookupV2Response`].
    /// This method will fail if any of the following fields are not set:
    /// - [`astronomy`](AstronomyLookupV2ResponseBuilder::astronomy)
    pub fn build(self) -> Result<AstronomyLookupV2Response, BuildError> {
        Ok(AstronomyLookupV2Response {
            ip: self.ip,
            location: self.location,
            astronomy: self
                .astronomy
                .ok_or_else(|| BuildError::missing_field("astronomy"))?,
        })
    }
}
