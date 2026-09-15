pub use crate::prelude::*;

/// Airport information, present when queried by IATA or ICAO code.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimezoneLookupV2ResponseAirportDetails {
    /// Classification of the airport based on size and traffic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The full name of the airport.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The latitude coordinate of the airport.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub latitude: Option<f64>,
    /// The longitude coordinate of the airport.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub longitude: Option<f64>,
    /// The elevation of the airport above sea level, measured in feet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevation_ft: Option<i64>,
    /// The two-letter code of the continent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent_code: Option<String>,
    /// The ISO 3166-1 alpha-2 code for the country where the airport is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// Code of the state/province/region where the airport is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_code: Option<String>,
    /// The city or administrative region that the airport serves.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The three-letter IATA airport code (e.g., LHR).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iata_code: Option<String>,
    /// The four-letter ICAO airport code (e.g., EGLL).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icao_code: Option<String>,
    /// The FAA location identifier, used primarily in the United States.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faa_code: Option<String>,
}

impl TimezoneLookupV2ResponseAirportDetails {
    pub fn builder() -> TimezoneLookupV2ResponseAirportDetailsBuilder {
        <TimezoneLookupV2ResponseAirportDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimezoneLookupV2ResponseAirportDetailsBuilder {
    r#type: Option<String>,
    name: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    elevation_ft: Option<i64>,
    continent_code: Option<String>,
    country_code: Option<String>,
    state_code: Option<String>,
    city: Option<String>,
    iata_code: Option<String>,
    icao_code: Option<String>,
    faa_code: Option<String>,
}

impl TimezoneLookupV2ResponseAirportDetailsBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn latitude(mut self, value: f64) -> Self {
        self.latitude = Some(value);
        self
    }

    pub fn longitude(mut self, value: f64) -> Self {
        self.longitude = Some(value);
        self
    }

    pub fn elevation_ft(mut self, value: i64) -> Self {
        self.elevation_ft = Some(value);
        self
    }

    pub fn continent_code(mut self, value: impl Into<String>) -> Self {
        self.continent_code = Some(value.into());
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn state_code(mut self, value: impl Into<String>) -> Self {
        self.state_code = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
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

    pub fn faa_code(mut self, value: impl Into<String>) -> Self {
        self.faa_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimezoneLookupV2ResponseAirportDetails`].
    pub fn build(self) -> Result<TimezoneLookupV2ResponseAirportDetails, BuildError> {
        Ok(TimezoneLookupV2ResponseAirportDetails {
            r#type: self.r#type,
            name: self.name,
            latitude: self.latitude,
            longitude: self.longitude,
            elevation_ft: self.elevation_ft,
            continent_code: self.continent_code,
            country_code: self.country_code,
            state_code: self.state_code,
            city: self.city,
            iata_code: self.iata_code,
            icao_code: self.icao_code,
            faa_code: self.faa_code,
        })
    }
}
