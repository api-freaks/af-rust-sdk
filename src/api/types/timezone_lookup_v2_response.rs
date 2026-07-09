pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimezoneLookupV2Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<TimezoneLookupV2ResponseLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<TimezoneLookupV2ResponseTimeZone>,
    #[serde(rename = "airport_detail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub airport_detail: Option<TimezoneLookupV2ResponseAirportDetails>,
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
    location: Option<TimezoneLookupV2ResponseLocation>,
    time_zone: Option<TimezoneLookupV2ResponseTimeZone>,
    airport_detail: Option<TimezoneLookupV2ResponseAirportDetails>,
    lo_code_details: Option<TimezoneLookupV2ResponseLoCodeDetails>,
}

impl TimezoneLookupV2ResponseBuilder {
    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn location(mut self, value: TimezoneLookupV2ResponseLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn time_zone(mut self, value: TimezoneLookupV2ResponseTimeZone) -> Self {
        self.time_zone = Some(value);
        self
    }

    pub fn airport_detail(mut self, value: TimezoneLookupV2ResponseAirportDetails) -> Self {
        self.airport_detail = Some(value);
        self
    }

    pub fn lo_code_details(mut self, value: TimezoneLookupV2ResponseLoCodeDetails) -> Self {
        self.lo_code_details = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimezoneLookupV2Response`].
    pub fn build(self) -> Result<TimezoneLookupV2Response, BuildError> {
        Ok(TimezoneLookupV2Response {
            ip: self.ip,
            location: self.location,
            time_zone: self.time_zone,
            airport_detail: self.airport_detail,
            lo_code_details: self.lo_code_details,
        })
    }
}
