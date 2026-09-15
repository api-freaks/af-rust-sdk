pub use crate::prelude::*;

/// Country-specific metadata.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeolocationLookupV2ResponseCountryMetadata {
    /// Calling code/Dialing code of the country.
    #[serde(default)]
    pub calling_code: String,
    /// Top Level Domain Name (TLD) of the country, which is also called ccTLD.
    #[serde(default)]
    pub tld: String,
    /// List of the languages' codes, spoken in the country.
    #[serde(default)]
    pub languages: Vec<String>,
}

impl GeolocationLookupV2ResponseCountryMetadata {
    pub fn builder() -> GeolocationLookupV2ResponseCountryMetadataBuilder {
        <GeolocationLookupV2ResponseCountryMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeolocationLookupV2ResponseCountryMetadataBuilder {
    calling_code: Option<String>,
    tld: Option<String>,
    languages: Option<Vec<String>>,
}

impl GeolocationLookupV2ResponseCountryMetadataBuilder {
    pub fn calling_code(mut self, value: impl Into<String>) -> Self {
        self.calling_code = Some(value.into());
        self
    }

    pub fn tld(mut self, value: impl Into<String>) -> Self {
        self.tld = Some(value.into());
        self
    }

    pub fn languages(mut self, value: Vec<String>) -> Self {
        self.languages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeolocationLookupV2ResponseCountryMetadata`].
    /// This method will fail if any of the following fields are not set:
    /// - [`calling_code`](GeolocationLookupV2ResponseCountryMetadataBuilder::calling_code)
    /// - [`tld`](GeolocationLookupV2ResponseCountryMetadataBuilder::tld)
    /// - [`languages`](GeolocationLookupV2ResponseCountryMetadataBuilder::languages)
    pub fn build(self) -> Result<GeolocationLookupV2ResponseCountryMetadata, BuildError> {
        Ok(GeolocationLookupV2ResponseCountryMetadata {
            calling_code: self
                .calling_code
                .ok_or_else(|| BuildError::missing_field("calling_code"))?,
            tld: self.tld.ok_or_else(|| BuildError::missing_field("tld"))?,
            languages: self
                .languages
                .ok_or_else(|| BuildError::missing_field("languages"))?,
        })
    }
}
