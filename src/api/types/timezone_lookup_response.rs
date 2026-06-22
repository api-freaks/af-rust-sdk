pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimezoneLookupResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<TimezoneLookupResponseLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<TimezoneLookupResponseTimeZone>,
    #[serde(rename = "airport_detail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub airport_detail: Option<TimezoneLookupResponseAirportDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lo_code_details: Option<TimezoneLookupResponseLoCodeDetails>,
}

impl TimezoneLookupResponse {
    pub fn builder() -> TimezoneLookupResponseBuilder {
        <TimezoneLookupResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimezoneLookupResponseBuilder {
    ip: Option<String>,
    location: Option<TimezoneLookupResponseLocation>,
    time_zone: Option<TimezoneLookupResponseTimeZone>,
    airport_detail: Option<TimezoneLookupResponseAirportDetails>,
    lo_code_details: Option<TimezoneLookupResponseLoCodeDetails>,
}

impl TimezoneLookupResponseBuilder {
    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn location(mut self, value: TimezoneLookupResponseLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn time_zone(mut self, value: TimezoneLookupResponseTimeZone) -> Self {
        self.time_zone = Some(value);
        self
    }

    pub fn airport_detail(mut self, value: TimezoneLookupResponseAirportDetails) -> Self {
        self.airport_detail = Some(value);
        self
    }

    pub fn lo_code_details(mut self, value: TimezoneLookupResponseLoCodeDetails) -> Self {
        self.lo_code_details = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimezoneLookupResponse`].
    pub fn build(self) -> Result<TimezoneLookupResponse, BuildError> {
        Ok(TimezoneLookupResponse {
            ip: self.ip,
            location: self.location,
            time_zone: self.time_zone,
            airport_detail: self.airport_detail,
            lo_code_details: self.lo_code_details,
        })
    }
}
