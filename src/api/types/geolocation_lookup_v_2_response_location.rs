pub use crate::prelude::*;

/// Geographic location information for the IP.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeolocationLookupV2ResponseLocation {
    /// 2-letter code of the continent.
    #[serde(default)]
    pub continent_code: String,
    /// Name of the continent.
    #[serde(default)]
    pub continent_name: String,
    /// Country code (ISO 3166-1 alpha-2) of the country.
    #[serde(default)]
    pub country_code2: String,
    /// Country code (ISO 3166-1 alpha-3) of the country.
    #[serde(default)]
    pub country_code3: String,
    /// Name of the country.
    #[serde(default)]
    pub country_name: String,
    /// Official name (ISO 3166) of the country.
    #[serde(default)]
    pub country_name_official: String,
    /// Name of the country's capital.
    #[serde(default)]
    pub country_capital: String,
    /// Name of the state/province/region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_prov: Option<String>,
    /// Code of the state/province/region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_code: Option<String>,
    /// Name of the district or county.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    /// Name of the city.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// A more specific area in city or it can be same as city.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// Circular radius in Km, where the IP address location can be found.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy_radius: Option<String>,
    /// Confidence level of the location match (e.g., low, medium, high).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<String>,
    /// Designated Market Area (DMA) code used in the United States for media marketing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dma_code: Option<String>,
    /// ZIP/Postal code of the place.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zipcode: Option<String>,
    /// Latitude of the place.
    #[serde(default)]
    pub latitude: String,
    /// Longitude of the place.
    #[serde(default)]
    pub longitude: String,
    /// Is the country belong to European Union?
    #[serde(default)]
    pub is_eu: bool,
    /// URL to get the country flag.
    #[serde(default)]
    pub country_flag: String,
    /// Geoname ID of the place from geonames.org.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geoname_id: Option<String>,
    /// Emoji of the Country flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_emoji: Option<String>,
}

impl GeolocationLookupV2ResponseLocation {
    pub fn builder() -> GeolocationLookupV2ResponseLocationBuilder {
        <GeolocationLookupV2ResponseLocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeolocationLookupV2ResponseLocationBuilder {
    continent_code: Option<String>,
    continent_name: Option<String>,
    country_code2: Option<String>,
    country_code3: Option<String>,
    country_name: Option<String>,
    country_name_official: Option<String>,
    country_capital: Option<String>,
    state_prov: Option<String>,
    state_code: Option<String>,
    district: Option<String>,
    city: Option<String>,
    locality: Option<String>,
    accuracy_radius: Option<String>,
    confidence: Option<String>,
    dma_code: Option<String>,
    zipcode: Option<String>,
    latitude: Option<String>,
    longitude: Option<String>,
    is_eu: Option<bool>,
    country_flag: Option<String>,
    geoname_id: Option<String>,
    country_emoji: Option<String>,
}

impl GeolocationLookupV2ResponseLocationBuilder {
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

    pub fn country_capital(mut self, value: impl Into<String>) -> Self {
        self.country_capital = Some(value.into());
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

    pub fn locality(mut self, value: impl Into<String>) -> Self {
        self.locality = Some(value.into());
        self
    }

    pub fn accuracy_radius(mut self, value: impl Into<String>) -> Self {
        self.accuracy_radius = Some(value.into());
        self
    }

    pub fn confidence(mut self, value: impl Into<String>) -> Self {
        self.confidence = Some(value.into());
        self
    }

    pub fn dma_code(mut self, value: impl Into<String>) -> Self {
        self.dma_code = Some(value.into());
        self
    }

    pub fn zipcode(mut self, value: impl Into<String>) -> Self {
        self.zipcode = Some(value.into());
        self
    }

    pub fn latitude(mut self, value: impl Into<String>) -> Self {
        self.latitude = Some(value.into());
        self
    }

    pub fn longitude(mut self, value: impl Into<String>) -> Self {
        self.longitude = Some(value.into());
        self
    }

    pub fn is_eu(mut self, value: bool) -> Self {
        self.is_eu = Some(value);
        self
    }

    pub fn country_flag(mut self, value: impl Into<String>) -> Self {
        self.country_flag = Some(value.into());
        self
    }

    pub fn geoname_id(mut self, value: impl Into<String>) -> Self {
        self.geoname_id = Some(value.into());
        self
    }

    pub fn country_emoji(mut self, value: impl Into<String>) -> Self {
        self.country_emoji = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GeolocationLookupV2ResponseLocation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`continent_code`](GeolocationLookupV2ResponseLocationBuilder::continent_code)
    /// - [`continent_name`](GeolocationLookupV2ResponseLocationBuilder::continent_name)
    /// - [`country_code2`](GeolocationLookupV2ResponseLocationBuilder::country_code2)
    /// - [`country_code3`](GeolocationLookupV2ResponseLocationBuilder::country_code3)
    /// - [`country_name`](GeolocationLookupV2ResponseLocationBuilder::country_name)
    /// - [`country_name_official`](GeolocationLookupV2ResponseLocationBuilder::country_name_official)
    /// - [`country_capital`](GeolocationLookupV2ResponseLocationBuilder::country_capital)
    /// - [`latitude`](GeolocationLookupV2ResponseLocationBuilder::latitude)
    /// - [`longitude`](GeolocationLookupV2ResponseLocationBuilder::longitude)
    /// - [`is_eu`](GeolocationLookupV2ResponseLocationBuilder::is_eu)
    /// - [`country_flag`](GeolocationLookupV2ResponseLocationBuilder::country_flag)
    pub fn build(self) -> Result<GeolocationLookupV2ResponseLocation, BuildError> {
        Ok(GeolocationLookupV2ResponseLocation {
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
            country_capital: self
                .country_capital
                .ok_or_else(|| BuildError::missing_field("country_capital"))?,
            state_prov: self.state_prov,
            state_code: self.state_code,
            district: self.district,
            city: self.city,
            locality: self.locality,
            accuracy_radius: self.accuracy_radius,
            confidence: self.confidence,
            dma_code: self.dma_code,
            zipcode: self.zipcode,
            latitude: self
                .latitude
                .ok_or_else(|| BuildError::missing_field("latitude"))?,
            longitude: self
                .longitude
                .ok_or_else(|| BuildError::missing_field("longitude"))?,
            is_eu: self
                .is_eu
                .ok_or_else(|| BuildError::missing_field("is_eu"))?,
            country_flag: self
                .country_flag
                .ok_or_else(|| BuildError::missing_field("country_flag"))?,
            geoname_id: self.geoname_id,
            country_emoji: self.country_emoji,
        })
    }
}
