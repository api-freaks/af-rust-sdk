pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeolocationLookupResponseCountryMetadata {
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

impl GeolocationLookupResponseCountryMetadata {
    pub fn builder() -> GeolocationLookupResponseCountryMetadataBuilder {
        <GeolocationLookupResponseCountryMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeolocationLookupResponseCountryMetadataBuilder {
    calling_code: Option<String>,
    tld: Option<String>,
    languages: Option<Vec<String>>,
}

impl GeolocationLookupResponseCountryMetadataBuilder {
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

    /// Consumes the builder and constructs a [`GeolocationLookupResponseCountryMetadata`].
    pub fn build(self) -> Result<GeolocationLookupResponseCountryMetadata, BuildError> {
        Ok(GeolocationLookupResponseCountryMetadata {
            calling_code: self.calling_code,
            tld: self.tld,
            languages: self.languages,
        })
    }
}
