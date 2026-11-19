pub use crate::prelude::*;

/// A single country supported by the SWIFT/BIC Code Lookup and Finder APIs.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatSupportedCountriesResponseSwiftSupportedCountriesItem {
    /// Full country name.
    #[serde(default)]
    pub name: String,
    /// ISO 3166-1 alpha-2 country code.
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
}

impl VatSupportedCountriesResponseSwiftSupportedCountriesItem {
    pub fn builder() -> VatSupportedCountriesResponseSwiftSupportedCountriesItemBuilder {
        <VatSupportedCountriesResponseSwiftSupportedCountriesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatSupportedCountriesResponseSwiftSupportedCountriesItemBuilder {
    name: Option<String>,
    country_code: Option<String>,
}

impl VatSupportedCountriesResponseSwiftSupportedCountriesItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VatSupportedCountriesResponseSwiftSupportedCountriesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](VatSupportedCountriesResponseSwiftSupportedCountriesItemBuilder::name)
    /// - [`country_code`](VatSupportedCountriesResponseSwiftSupportedCountriesItemBuilder::country_code)
    pub fn build(
        self,
    ) -> Result<VatSupportedCountriesResponseSwiftSupportedCountriesItem, BuildError> {
        Ok(VatSupportedCountriesResponseSwiftSupportedCountriesItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
        })
    }
}
