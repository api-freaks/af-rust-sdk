pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkCurrentWeatherResponseBulkItemLocationZero {
    /// Geographic latitude coordinate in decimal degrees, ranging from -90 (South Pole) to +90 (North Pole).
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub latitude: f64,
    /// Geographic longitude coordinate in decimal degrees, ranging from -180 (West) to +180 (East).
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub longitude: f64,
    /// Full name of the country corresponding to the provided coordinates.
    #[serde(default)]
    pub country_name: String,
    /// State, province, or primary administrative division name for the location.
    #[serde(default)]
    pub state_prov: String,
    /// City or municipal area name associated with the coordinate location.
    #[serde(default)]
    pub city: String,
    /// Specific locality, neighborhood, district, or village name within the broader area.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// Height above mean sea level in meters for the specified coordinates.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub elevation: Option<f64>,
    /// IANA timezone database identifier for the location (e.g., America/New_York, Europe/London).
    #[serde(default)]
    pub timezone: String,
    /// Abbreviated timezone representation based on current offset (e.g., EST, GMT, PST).
    #[serde(default)]
    pub timezone_abbreviation: String,
}

impl BulkCurrentWeatherResponseBulkItemLocationZero {
    pub fn builder() -> BulkCurrentWeatherResponseBulkItemLocationZeroBuilder {
        <BulkCurrentWeatherResponseBulkItemLocationZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkCurrentWeatherResponseBulkItemLocationZeroBuilder {
    latitude: Option<f64>,
    longitude: Option<f64>,
    country_name: Option<String>,
    state_prov: Option<String>,
    city: Option<String>,
    locality: Option<String>,
    elevation: Option<f64>,
    timezone: Option<String>,
    timezone_abbreviation: Option<String>,
}

impl BulkCurrentWeatherResponseBulkItemLocationZeroBuilder {
    pub fn latitude(mut self, value: f64) -> Self {
        self.latitude = Some(value);
        self
    }

    pub fn longitude(mut self, value: f64) -> Self {
        self.longitude = Some(value);
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

    /// Consumes the builder and constructs a [`BulkCurrentWeatherResponseBulkItemLocationZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`latitude`](BulkCurrentWeatherResponseBulkItemLocationZeroBuilder::latitude)
    /// - [`longitude`](BulkCurrentWeatherResponseBulkItemLocationZeroBuilder::longitude)
    /// - [`country_name`](BulkCurrentWeatherResponseBulkItemLocationZeroBuilder::country_name)
    /// - [`state_prov`](BulkCurrentWeatherResponseBulkItemLocationZeroBuilder::state_prov)
    /// - [`city`](BulkCurrentWeatherResponseBulkItemLocationZeroBuilder::city)
    /// - [`timezone`](BulkCurrentWeatherResponseBulkItemLocationZeroBuilder::timezone)
    /// - [`timezone_abbreviation`](BulkCurrentWeatherResponseBulkItemLocationZeroBuilder::timezone_abbreviation)
    pub fn build(self) -> Result<BulkCurrentWeatherResponseBulkItemLocationZero, BuildError> {
        Ok(BulkCurrentWeatherResponseBulkItemLocationZero {
            latitude: self
                .latitude
                .ok_or_else(|| BuildError::missing_field("latitude"))?,
            longitude: self
                .longitude
                .ok_or_else(|| BuildError::missing_field("longitude"))?,
            country_name: self
                .country_name
                .ok_or_else(|| BuildError::missing_field("country_name"))?,
            state_prov: self
                .state_prov
                .ok_or_else(|| BuildError::missing_field("state_prov"))?,
            city: self.city.ok_or_else(|| BuildError::missing_field("city"))?,
            locality: self.locality,
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
