pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WeatherTimeSeriesResponseLocationCity {
    /// Original location query string as submitted, including full address or place name.
    #[serde(default)]
    pub location_string: String,
    /// Resolved country name derived from the geocoded location query.
    #[serde(default)]
    pub country_name: String,
    /// State, province, or primary administrative division identified from the location.
    #[serde(default)]
    pub state_prov: String,
    /// City or municipal area name extracted from the geocoded location.
    #[serde(default)]
    pub city: String,
    /// Specific locality, neighborhood, suburb, or village within the geocoded area.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// Geocoded latitude coordinate in decimal degrees, ranging from -90 to +90.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub latitude: f64,
    /// Geocoded longitude coordinate in decimal degrees, ranging from -180 to +180.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub longitude: f64,
    /// Elevation above mean sea level in meters at the geocoded coordinates.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub elevation: Option<f64>,
    /// IANA timezone database identifier for the geocoded location (e.g., America/Los_Angeles).
    #[serde(default)]
    pub timezone: String,
    /// Current timezone abbreviation for the location based on local offset (e.g., PDT, CET).
    #[serde(default)]
    pub timezone_abbreviation: String,
}

impl WeatherTimeSeriesResponseLocationCity {
    pub fn builder() -> WeatherTimeSeriesResponseLocationCityBuilder {
        <WeatherTimeSeriesResponseLocationCityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WeatherTimeSeriesResponseLocationCityBuilder {
    location_string: Option<String>,
    country_name: Option<String>,
    state_prov: Option<String>,
    city: Option<String>,
    locality: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    elevation: Option<f64>,
    timezone: Option<String>,
    timezone_abbreviation: Option<String>,
}

impl WeatherTimeSeriesResponseLocationCityBuilder {
    pub fn location_string(mut self, value: impl Into<String>) -> Self {
        self.location_string = Some(value.into());
        self
    }

    pub fn country_name(mut self, value: impl Into<String>) -> Self {
        self.country_name = Some(value.into());
        self
    }

    pub fn state_prov(mut self, value: impl Into<String>) -> Self {
        self.state_prov = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn locality(mut self, value: impl Into<String>) -> Self {
        self.locality = Some(value.into());
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

    pub fn elevation(mut self, value: f64) -> Self {
        self.elevation = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    pub fn timezone_abbreviation(mut self, value: impl Into<String>) -> Self {
        self.timezone_abbreviation = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WeatherTimeSeriesResponseLocationCity`].
    /// This method will fail if any of the following fields are not set:
    /// - [`location_string`](WeatherTimeSeriesResponseLocationCityBuilder::location_string)
    /// - [`country_name`](WeatherTimeSeriesResponseLocationCityBuilder::country_name)
    /// - [`state_prov`](WeatherTimeSeriesResponseLocationCityBuilder::state_prov)
    /// - [`city`](WeatherTimeSeriesResponseLocationCityBuilder::city)
    /// - [`latitude`](WeatherTimeSeriesResponseLocationCityBuilder::latitude)
    /// - [`longitude`](WeatherTimeSeriesResponseLocationCityBuilder::longitude)
    /// - [`timezone`](WeatherTimeSeriesResponseLocationCityBuilder::timezone)
    /// - [`timezone_abbreviation`](WeatherTimeSeriesResponseLocationCityBuilder::timezone_abbreviation)
    pub fn build(self) -> Result<WeatherTimeSeriesResponseLocationCity, BuildError> {
        Ok(WeatherTimeSeriesResponseLocationCity {
            location_string: self
                .location_string
                .ok_or_else(|| BuildError::missing_field("location_string"))?,
            country_name: self
                .country_name
                .ok_or_else(|| BuildError::missing_field("country_name"))?,
            state_prov: self
                .state_prov
                .ok_or_else(|| BuildError::missing_field("state_prov"))?,
            city: self.city.ok_or_else(|| BuildError::missing_field("city"))?,
            locality: self.locality,
            latitude: self
                .latitude
                .ok_or_else(|| BuildError::missing_field("latitude"))?,
            longitude: self
                .longitude
                .ok_or_else(|| BuildError::missing_field("longitude"))?,
            elevation: self.elevation,
            timezone: self
                .timezone
                .ok_or_else(|| BuildError::missing_field("timezone"))?,
            timezone_abbreviation: self
                .timezone_abbreviation
                .ok_or_else(|| BuildError::missing_field("timezone_abbreviation"))?,
        })
    }
}
