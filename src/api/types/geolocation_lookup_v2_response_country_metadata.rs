pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeolocationLookupV2ResponseCountryMetadata {
    /// The calling code of the country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calling_code: Option<String>,
    /// The top level domain of the country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tld: Option<String>,
    /// The languages spoken in the country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
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
    pub fn build(self) -> Result<GeolocationLookupV2ResponseCountryMetadata, BuildError> {
        Ok(GeolocationLookupV2ResponseCountryMetadata {
            calling_code: self.calling_code,
            tld: self.tld,
            languages: self.languages,
        })
    }
}
