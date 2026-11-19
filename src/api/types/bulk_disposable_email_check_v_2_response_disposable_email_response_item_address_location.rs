pub use crate::prelude::*;

/// Geolocation data derived from the IP address.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressLocation {
    /// Municipal or city name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// District or neighborhood.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    /// Postal or ZIP code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zipcode: Option<String>,
    /// State or province.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_prov: Option<String>,
    /// Common country name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>,
    /// Official country name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name_official: Option<String>,
    /// Two-character ISO country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code2: Option<String>,
    /// Three-character ISO country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code3: Option<String>,
    /// Continental region name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent_name: Option<String>,
    /// Continental region identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent_code: Option<String>,
    /// Geolocation precision radius. Not included for every IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy_radius: Option<String>,
    /// Confidence score for geolocation accuracy. Not included for every IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<String>,
    /// European Union membership status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_eu: Option<bool>,
}

impl BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressLocation {
    pub fn builder(
    ) -> BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressLocationBuilder {
        <BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressLocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressLocationBuilder {
    city: Option<String>,
    district: Option<String>,
    zipcode: Option<String>,
    state_prov: Option<String>,
    country_name: Option<String>,
    country_name_official: Option<String>,
    country_code2: Option<String>,
    country_code3: Option<String>,
    continent_name: Option<String>,
    continent_code: Option<String>,
    accuracy_radius: Option<String>,
    confidence: Option<String>,
    is_eu: Option<bool>,
}

impl BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressLocationBuilder {
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn district(mut self, value: impl Into<String>) -> Self {
        self.district = Some(value.into());
        self
    }

    pub fn zipcode(mut self, value: impl Into<String>) -> Self {
        self.zipcode = Some(value.into());
        self
    }

    pub fn state_prov(mut self, value: impl Into<String>) -> Self {
        self.state_prov = Some(value.into());
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

    pub fn country_code2(mut self, value: impl Into<String>) -> Self {
        self.country_code2 = Some(value.into());
        self
    }

    pub fn country_code3(mut self, value: impl Into<String>) -> Self {
        self.country_code3 = Some(value.into());
        self
    }

    pub fn continent_name(mut self, value: impl Into<String>) -> Self {
        self.continent_name = Some(value.into());
        self
    }

    pub fn continent_code(mut self, value: impl Into<String>) -> Self {
        self.continent_code = Some(value.into());
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

    pub fn is_eu(mut self, value: bool) -> Self {
        self.is_eu = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressLocation`].
    pub fn build(
        self,
    ) -> Result<
        BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressLocation,
        BuildError,
    > {
        Ok(
            BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddressLocation {
                city: self.city,
                district: self.district,
                zipcode: self.zipcode,
                state_prov: self.state_prov,
                country_name: self.country_name,
                country_name_official: self.country_name_official,
                country_code2: self.country_code2,
                country_code3: self.country_code3,
                continent_name: self.continent_name,
                continent_code: self.continent_code,
                accuracy_radius: self.accuracy_radius,
                confidence: self.confidence,
                is_eu: self.is_eu,
            },
        )
    }
}
