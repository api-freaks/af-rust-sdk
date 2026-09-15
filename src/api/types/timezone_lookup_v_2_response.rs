pub use crate::prelude::*;

/// Timezone lookup result. time_zone is always present. Exactly which other object accompanies it depends on the lookup mode: tz name and lat/long coordinates return time_zone only (no location, no ip); location address returns a basic location object; ip param or client-IP fallback returns a rich location object plus top-level ip; iata_code/icao_code returns airport_details instead of location; lo_code returns lo_code_details instead of location.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimezoneLookupV2Response {
    /// The IP address used for the timezone lookup. Present when queried using the ip parameter, or with no location-identifying parameter at all (client-IP fallback). Absent for every other lookup mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Timezone and date/time information for the location.
    #[serde(default)]
    pub time_zone: TimezoneLookupV2ResponseTimeZone,
    /// Geographic location information. Only present for location (address) and ip (or client-IP fallback) lookups; absent for tz, lat/long, iata_code/icao_code, and lo_code lookups. Field set varies by mode: location returns location_string plus a basic field set (country_name, state_prov, city, locality, latitude, longitude); ip/default returns a richer geo-IP field set (continent_code, continent_name, country_code2, country_code3, country_name_official, is_eu, state_code, district, zipcode) plus the common fields, but never location_string or locality.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<TimezoneLookupV2ResponseLocation>,
    /// Airport information, present when queried by IATA or ICAO code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub airport_details: Option<TimezoneLookupV2ResponseAirportDetails>,
    /// UN/LOCODE location details, present when queried by LO code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lo_code_details: Option<TimezoneLookupV2ResponseLoCodeDetails>,
}

impl TimezoneLookupV2Response {
    pub fn builder() -> TimezoneLookupV2ResponseBuilder {
        <TimezoneLookupV2ResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimezoneLookupV2ResponseBuilder {
    ip: Option<String>,
    time_zone: Option<TimezoneLookupV2ResponseTimeZone>,
    location: Option<TimezoneLookupV2ResponseLocation>,
    airport_details: Option<TimezoneLookupV2ResponseAirportDetails>,
    lo_code_details: Option<TimezoneLookupV2ResponseLoCodeDetails>,
}

impl TimezoneLookupV2ResponseBuilder {
    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn time_zone(mut self, value: TimezoneLookupV2ResponseTimeZone) -> Self {
        self.time_zone = Some(value);
        self
    }

    pub fn location(mut self, value: TimezoneLookupV2ResponseLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn airport_details(mut self, value: TimezoneLookupV2ResponseAirportDetails) -> Self {
        self.airport_details = Some(value);
        self
    }

    pub fn lo_code_details(mut self, value: TimezoneLookupV2ResponseLoCodeDetails) -> Self {
        self.lo_code_details = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimezoneLookupV2Response`].
    /// This method will fail if any of the following fields are not set:
    /// - [`time_zone`](TimezoneLookupV2ResponseBuilder::time_zone)
    pub fn build(self) -> Result<TimezoneLookupV2Response, BuildError> {
        Ok(TimezoneLookupV2Response {
            ip: self.ip,
            time_zone: self
                .time_zone
                .ok_or_else(|| BuildError::missing_field("time_zone"))?,
            location: self.location,
            airport_details: self.airport_details,
            lo_code_details: self.lo_code_details,
        })
    }
}
