pub use crate::prelude::*;

/// Query parameters for timezone_lookup_v2
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimezoneLookupV2QueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response. Possible values: json, xml.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<TimezoneLookupV2RequestFormat>,
    /// IPv4 or IPv6 address to extract timezone information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Timezone name in IANA format (e.g., Asia/Kolkata) to retrieve information directly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tz: Option<String>,
    /// Location string (preferably city and country) to extract timezone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Latitude for geolocation-based timezone lookup. Only time_zone is returned for this mode; no location object is included.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub lat: Option<f64>,
    /// Longitude for geolocation-based timezone lookup. Only time_zone is returned for this mode; no location object is included.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub long: Option<f64>,
    /// Response language for location fields. Default: en.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<TimezoneLookupV2RequestLang>,
    /// 3-letter IATA airport code (e.g., LHR) to extract timezone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iata_code: Option<String>,
    /// 4-letter ICAO airport code (e.g., KJFK) to extract timezone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icao_code: Option<String>,
    /// 5-letter UN/LOCODE city code to extract timezone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lo_code: Option<String>,
}

impl TimezoneLookupV2QueryRequest {
    pub fn builder() -> TimezoneLookupV2QueryRequestBuilder {
        <TimezoneLookupV2QueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimezoneLookupV2QueryRequestBuilder {
    api_key: Option<String>,
    format: Option<TimezoneLookupV2RequestFormat>,
    ip: Option<String>,
    tz: Option<String>,
    location: Option<String>,
    lat: Option<f64>,
    long: Option<f64>,
    lang: Option<TimezoneLookupV2RequestLang>,
    iata_code: Option<String>,
    icao_code: Option<String>,
    lo_code: Option<String>,
}

impl TimezoneLookupV2QueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: TimezoneLookupV2RequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn tz(mut self, value: impl Into<String>) -> Self {
        self.tz = Some(value.into());
        self
    }

    pub fn location(mut self, value: impl Into<String>) -> Self {
        self.location = Some(value.into());
        self
    }

    pub fn lat(mut self, value: f64) -> Self {
        self.lat = Some(value);
        self
    }

    pub fn long(mut self, value: f64) -> Self {
        self.long = Some(value);
        self
    }

    pub fn lang(mut self, value: TimezoneLookupV2RequestLang) -> Self {
        self.lang = Some(value);
        self
    }

    pub fn iata_code(mut self, value: impl Into<String>) -> Self {
        self.iata_code = Some(value.into());
        self
    }

    pub fn icao_code(mut self, value: impl Into<String>) -> Self {
        self.icao_code = Some(value.into());
        self
    }

    pub fn lo_code(mut self, value: impl Into<String>) -> Self {
        self.lo_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimezoneLookupV2QueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](TimezoneLookupV2QueryRequestBuilder::api_key)
    pub fn build(self) -> Result<TimezoneLookupV2QueryRequest, BuildError> {
        Ok(TimezoneLookupV2QueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            ip: self.ip,
            tz: self.tz,
            location: self.location,
            lat: self.lat,
            long: self.long,
            lang: self.lang,
            iata_code: self.iata_code,
            icao_code: self.icao_code,
            lo_code: self.lo_code,
        })
    }
}
