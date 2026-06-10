pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MarineWeatherResponseLocationContinentCode {
    /// Two-letter continent code (e.g., NA for North America, EU for Europe, AS for Asia).
    #[serde(default)]
    pub continent_code: String,
    /// Full name of the continent where the IP address is registered.
    #[serde(default)]
    pub continent_name: String,
    /// ISO 3166-1 alpha-2 two-letter country code (e.g., US, GB, FR).
    #[serde(default)]
    pub country_code2: String,
    /// ISO 3166-1 alpha-3 three-letter country code (e.g., USA, GBR, FRA).
    #[serde(default)]
    pub country_code3: String,
    /// Common name of the country associated with the IP address.
    #[serde(default)]
    pub country_name: String,
    /// Official long-form country name as recognized internationally (e.g., United States of America).
    #[serde(default)]
    pub country_name_official: String,
    /// Boolean flag indicating whether the country is a member state of the European Union.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_eu: Option<bool>,
    /// State, province, or primary administrative division associated with the IP location.
    #[serde(default)]
    pub state_prov: String,
    /// ISO 3166-2 subdivision code for the state or province (e.g., CA for California).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_code: Option<String>,
    /// District, county, or secondary administrative division within the region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    /// City or urban area name where the IP address is geographically registered.
    #[serde(default)]
    pub city: String,
    /// Postal code or ZIP code for the approximate location of the IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zipcode: Option<String>,
    /// Geographic latitude in decimal degrees for the IP geolocation, ranging from -90 to +90.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub latitude: f64,
    /// Geographic longitude in decimal degrees for the IP geolocation, ranging from -180 to +180.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub longitude: f64,
    /// Specific locality, neighborhood, or small area designation within the city.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// Elevation above mean sea level in meters for the IP geolocation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub elevation: Option<f64>,
    /// IANA timezone database identifier for the IP location (e.g., America/Chicago, Asia/Tokyo).
    #[serde(default)]
    pub timezone: String,
    /// Current timezone abbreviation based on local offset (e.g., CST, JST, UTC).
    #[serde(default)]
    pub timezone_abbreviation: String,
}

impl MarineWeatherResponseLocationContinentCode {
    pub fn builder() -> MarineWeatherResponseLocationContinentCodeBuilder {
        <MarineWeatherResponseLocationContinentCodeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MarineWeatherResponseLocationContinentCodeBuilder {
    continent_code: Option<String>,
    continent_name: Option<String>,
    country_code2: Option<String>,
    country_code3: Option<String>,
    country_name: Option<String>,
    country_name_official: Option<String>,
    is_eu: Option<bool>,
    state_prov: Option<String>,
    state_code: Option<String>,
    district: Option<String>,
    city: Option<String>,
    zipcode: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    locality: Option<String>,
    elevation: Option<f64>,
    timezone: Option<String>,
    timezone_abbreviation: Option<String>,
}

impl MarineWeatherResponseLocationContinentCodeBuilder {
    pub fn continent_code(mut self, value: impl Into<String>) -> Self {
        self.continent_code = Some(value.into());
        self
    }

    pub fn continent_name(mut self, value: impl Into<String>) -> Self {
        self.continent_name = Some(value.into());
        self
    }

    pub fn country_code2(mut self, value: impl Into<String>) -> Self {
        self.country_code2 = Some(value.into());
        self
    }

    pub fn country_code3(mut self, value: impl Into<String>) -> Self {
        self.country_code3 = Some(value.into());
        self
    }

    pub fn country_name(mut self, value: impl Into<String>) -> Self {
        self.country_name = Some(value.into());
        self
    }

    pub fn country_name_official(mut self, value: impl Into<String>) -> Self {
        self.country_name_official = Some(value.into());
        self
    }

    pub fn is_eu(mut self, value: bool) -> Self {
        self.is_eu = Some(value);
        self
    }

    pub fn state_prov(mut self, value: impl Into<String>) -> Self {
        self.state_prov = Some(value.into());
        self
    }

    pub fn state_code(mut self, value: impl Into<String>) -> Self {
        self.state_code = Some(value.into());
        self
    }

    pub fn district(mut self, value: impl Into<String>) -> Self {
        self.district = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn zipcode(mut self, value: impl Into<String>) -> Self {
        self.zipcode = Some(value.into());
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

    /// Consumes the builder and constructs a [`MarineWeatherResponseLocationContinentCode`].
    /// This method will fail if any of the following fields are not set:
    /// - [`continent_code`](MarineWeatherResponseLocationContinentCodeBuilder::continent_code)
    /// - [`continent_name`](MarineWeatherResponseLocationContinentCodeBuilder::continent_name)
    /// - [`country_code2`](MarineWeatherResponseLocationContinentCodeBuilder::country_code2)
    /// - [`country_code3`](MarineWeatherResponseLocationContinentCodeBuilder::country_code3)
    /// - [`country_name`](MarineWeatherResponseLocationContinentCodeBuilder::country_name)
    /// - [`country_name_official`](MarineWeatherResponseLocationContinentCodeBuilder::country_name_official)
    /// - [`state_prov`](MarineWeatherResponseLocationContinentCodeBuilder::state_prov)
    /// - [`city`](MarineWeatherResponseLocationContinentCodeBuilder::city)
    /// - [`latitude`](MarineWeatherResponseLocationContinentCodeBuilder::latitude)
    /// - [`longitude`](MarineWeatherResponseLocationContinentCodeBuilder::longitude)
    /// - [`timezone`](MarineWeatherResponseLocationContinentCodeBuilder::timezone)
    /// - [`timezone_abbreviation`](MarineWeatherResponseLocationContinentCodeBuilder::timezone_abbreviation)
    pub fn build(self) -> Result<MarineWeatherResponseLocationContinentCode, BuildError> {
        Ok(MarineWeatherResponseLocationContinentCode {
            continent_code: self
                .continent_code
                .ok_or_else(|| BuildError::missing_field("continent_code"))?,
            continent_name: self
                .continent_name
                .ok_or_else(|| BuildError::missing_field("continent_name"))?,
            country_code2: self
                .country_code2
                .ok_or_else(|| BuildError::missing_field("country_code2"))?,
            country_code3: self
                .country_code3
                .ok_or_else(|| BuildError::missing_field("country_code3"))?,
            country_name: self
                .country_name
                .ok_or_else(|| BuildError::missing_field("country_name"))?,
            country_name_official: self
                .country_name_official
                .ok_or_else(|| BuildError::missing_field("country_name_official"))?,
            is_eu: self.is_eu,
            state_prov: self
                .state_prov
                .ok_or_else(|| BuildError::missing_field("state_prov"))?,
            state_code: self.state_code,
            district: self.district,
            city: self.city.ok_or_else(|| BuildError::missing_field("city"))?,
            zipcode: self.zipcode,
            latitude: self
                .latitude
                .ok_or_else(|| BuildError::missing_field("latitude"))?,
            longitude: self
                .longitude
                .ok_or_else(|| BuildError::missing_field("longitude"))?,
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
