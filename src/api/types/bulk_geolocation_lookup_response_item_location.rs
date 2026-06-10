pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkGeolocationLookupResponseItemLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name_official: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_capital: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_prov: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy_radius: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dma_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zipcode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_eu: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_flag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geoname_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_emoji: Option<String>,
}

impl BulkGeolocationLookupResponseItemLocation {
    pub fn builder() -> BulkGeolocationLookupResponseItemLocationBuilder {
        <BulkGeolocationLookupResponseItemLocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupResponseItemLocationBuilder {
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

impl BulkGeolocationLookupResponseItemLocationBuilder {
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

    /// Consumes the builder and constructs a [`BulkGeolocationLookupResponseItemLocation`].
    pub fn build(self) -> Result<BulkGeolocationLookupResponseItemLocation, BuildError> {
        Ok(BulkGeolocationLookupResponseItemLocation {
            continent_code: self.continent_code,
            continent_name: self.continent_name,
            country_code2: self.country_code2,
            country_code3: self.country_code3,
            country_name: self.country_name,
            country_name_official: self.country_name_official,
            country_capital: self.country_capital,
            state_prov: self.state_prov,
            state_code: self.state_code,
            district: self.district,
            city: self.city,
            locality: self.locality,
            accuracy_radius: self.accuracy_radius,
            confidence: self.confidence,
            dma_code: self.dma_code,
            zipcode: self.zipcode,
            latitude: self.latitude,
            longitude: self.longitude,
            is_eu: self.is_eu,
            country_flag: self.country_flag,
            geoname_id: self.geoname_id,
            country_emoji: self.country_emoji,
        })
    }
}
