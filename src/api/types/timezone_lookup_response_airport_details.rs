pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimezoneLookupResponseAirportDetails {
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub longitude: String,
    #[serde(default)]
    pub latitude: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub elevation_ft: f64,
    #[serde(default)]
    pub continent_code: String,
    #[serde(default)]
    pub country_code: String,
    #[serde(default)]
    pub state_code: String,
    #[serde(default)]
    pub city: String,
    #[serde(default)]
    pub iata_code: String,
    #[serde(default)]
    pub icao_code: String,
    #[serde(default)]
    pub faa_code: String,
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
    longitude: Option<String>,
    latitude: Option<String>,
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

    pub fn longitude(mut self, value: impl Into<String>) -> Self {
        self.longitude = Some(value.into());
        self
    }

    pub fn latitude(mut self, value: impl Into<String>) -> Self {
        self.latitude = Some(value.into());
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
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimezoneLookupResponseAirportDetailsBuilder::r#type)
    /// - [`name`](TimezoneLookupResponseAirportDetailsBuilder::name)
    /// - [`longitude`](TimezoneLookupResponseAirportDetailsBuilder::longitude)
    /// - [`latitude`](TimezoneLookupResponseAirportDetailsBuilder::latitude)
    /// - [`elevation_ft`](TimezoneLookupResponseAirportDetailsBuilder::elevation_ft)
    /// - [`continent_code`](TimezoneLookupResponseAirportDetailsBuilder::continent_code)
    /// - [`country_code`](TimezoneLookupResponseAirportDetailsBuilder::country_code)
    /// - [`state_code`](TimezoneLookupResponseAirportDetailsBuilder::state_code)
    /// - [`city`](TimezoneLookupResponseAirportDetailsBuilder::city)
    /// - [`iata_code`](TimezoneLookupResponseAirportDetailsBuilder::iata_code)
    /// - [`icao_code`](TimezoneLookupResponseAirportDetailsBuilder::icao_code)
    /// - [`faa_code`](TimezoneLookupResponseAirportDetailsBuilder::faa_code)
    pub fn build(self) -> Result<TimezoneLookupResponseAirportDetails, BuildError> {
        Ok(TimezoneLookupResponseAirportDetails {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            longitude: self
                .longitude
                .ok_or_else(|| BuildError::missing_field("longitude"))?,
            latitude: self
                .latitude
                .ok_or_else(|| BuildError::missing_field("latitude"))?,
            elevation_ft: self
                .elevation_ft
                .ok_or_else(|| BuildError::missing_field("elevation_ft"))?,
            continent_code: self
                .continent_code
                .ok_or_else(|| BuildError::missing_field("continent_code"))?,
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            state_code: self
                .state_code
                .ok_or_else(|| BuildError::missing_field("state_code"))?,
            city: self.city.ok_or_else(|| BuildError::missing_field("city"))?,
            iata_code: self
                .iata_code
                .ok_or_else(|| BuildError::missing_field("iata_code"))?,
            icao_code: self
                .icao_code
                .ok_or_else(|| BuildError::missing_field("icao_code"))?,
            faa_code: self
                .faa_code
                .ok_or_else(|| BuildError::missing_field("faa_code"))?,
        })
    }
}
