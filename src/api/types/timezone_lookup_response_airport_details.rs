pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimezoneLookupResponseAirportDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub elevation_ft: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iata_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icao_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faa_code: Option<String>,
}

impl TimezoneLookupResponseAirportDetails {
    pub fn builder() -> TimezoneLookupResponseAirportDetailsBuilder {
        <TimezoneLookupResponseAirportDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimezoneLookupResponseAirportDetailsBuilder {
    r#type: Option<String>,
    name: Option<String>,
    longitude: Option<f64>,
    latitude: Option<f64>,
    elevation_ft: Option<f64>,
    continent_code: Option<String>,
    country_code: Option<String>,
    state_code: Option<String>,
    city: Option<String>,
    iata_code: Option<String>,
    icao_code: Option<String>,
    faa_code: Option<String>,
}

impl TimezoneLookupResponseAirportDetailsBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn longitude(mut self, value: f64) -> Self {
        self.longitude = Some(value);
        self
    }

    pub fn latitude(mut self, value: f64) -> Self {
        self.latitude = Some(value);
        self
    }

    pub fn elevation_ft(mut self, value: f64) -> Self {
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

    /// Consumes the builder and constructs a [`TimezoneLookupResponseAirportDetails`].
    pub fn build(self) -> Result<TimezoneLookupResponseAirportDetails, BuildError> {
        Ok(TimezoneLookupResponseAirportDetails {
            r#type: self.r#type,
            name: self.name,
            longitude: self.longitude,
            latitude: self.latitude,
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
