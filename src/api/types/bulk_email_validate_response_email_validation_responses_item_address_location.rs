pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkEmailValidateResponseEmailValidationResponsesItemAddressLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zipcode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_prov: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name_official: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy_radius: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_eu: Option<bool>,
}

impl BulkEmailValidateResponseEmailValidationResponsesItemAddressLocation {
    pub fn builder() -> BulkEmailValidateResponseEmailValidationResponsesItemAddressLocationBuilder
    {
        <BulkEmailValidateResponseEmailValidationResponsesItemAddressLocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkEmailValidateResponseEmailValidationResponsesItemAddressLocationBuilder {
    city: Option<String>,
    district: Option<String>,
    confidence: Option<String>,
    zipcode: Option<String>,
    state_prov: Option<String>,
    country_name: Option<String>,
    continent_name: Option<String>,
    continent_code: Option<String>,
    country_code2: Option<String>,
    country_code3: Option<String>,
    country_name_official: Option<String>,
    accuracy_radius: Option<String>,
    is_eu: Option<bool>,
}

impl BulkEmailValidateResponseEmailValidationResponsesItemAddressLocationBuilder {
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn district(mut self, value: impl Into<String>) -> Self {
        self.district = Some(value.into());
        self
    }

    pub fn confidence(mut self, value: impl Into<String>) -> Self {
        self.confidence = Some(value.into());
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

    pub fn continent_name(mut self, value: impl Into<String>) -> Self {
        self.continent_name = Some(value.into());
        self
    }

    pub fn continent_code(mut self, value: impl Into<String>) -> Self {
        self.continent_code = Some(value.into());
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

    pub fn country_name_official(mut self, value: impl Into<String>) -> Self {
        self.country_name_official = Some(value.into());
        self
    }

    pub fn accuracy_radius(mut self, value: impl Into<String>) -> Self {
        self.accuracy_radius = Some(value.into());
        self
    }

    pub fn is_eu(mut self, value: bool) -> Self {
        self.is_eu = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkEmailValidateResponseEmailValidationResponsesItemAddressLocation`].
    pub fn build(
        self,
    ) -> Result<BulkEmailValidateResponseEmailValidationResponsesItemAddressLocation, BuildError>
    {
        Ok(
            BulkEmailValidateResponseEmailValidationResponsesItemAddressLocation {
                city: self.city,
                district: self.district,
                confidence: self.confidence,
                zipcode: self.zipcode,
                state_prov: self.state_prov,
                country_name: self.country_name,
                continent_name: self.continent_name,
                continent_code: self.continent_code,
                country_code2: self.country_code2,
                country_code3: self.country_code3,
                country_name_official: self.country_name_official,
                accuracy_radius: self.accuracy_radius,
                is_eu: self.is_eu,
            },
        )
    }
}
