pub use crate::prelude::*;

/// A single country supported by the IBAN Validation API.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatSupportedCountriesResponseIbanSupportedCountriesItem {
    /// ISO 3166-1 alpha-2 country code.
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    /// Full country name.
    #[serde(default)]
    pub name: String,
}

impl VatSupportedCountriesResponseIbanSupportedCountriesItem {
    pub fn builder() -> VatSupportedCountriesResponseIbanSupportedCountriesItemBuilder {
        <VatSupportedCountriesResponseIbanSupportedCountriesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatSupportedCountriesResponseIbanSupportedCountriesItemBuilder {
    country_code: Option<String>,
    name: Option<String>,
}

impl VatSupportedCountriesResponseIbanSupportedCountriesItemBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VatSupportedCountriesResponseIbanSupportedCountriesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](VatSupportedCountriesResponseIbanSupportedCountriesItemBuilder::country_code)
    /// - [`name`](VatSupportedCountriesResponseIbanSupportedCountriesItemBuilder::name)
    pub fn build(
        self,
    ) -> Result<VatSupportedCountriesResponseIbanSupportedCountriesItem, BuildError> {
        Ok(VatSupportedCountriesResponseIbanSupportedCountriesItem {
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
