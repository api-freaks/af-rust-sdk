pub use crate::prelude::*;

/// Country-specific metadata.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkGeolocationLookupV2ResponseItemAbuseCountryMetadata {
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

impl BulkGeolocationLookupV2ResponseItemAbuseCountryMetadata {
    pub fn builder() -> BulkGeolocationLookupV2ResponseItemAbuseCountryMetadataBuilder {
        <BulkGeolocationLookupV2ResponseItemAbuseCountryMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupV2ResponseItemAbuseCountryMetadataBuilder {
    calling_code: Option<String>,
    tld: Option<String>,
    languages: Option<Vec<String>>,
}

impl BulkGeolocationLookupV2ResponseItemAbuseCountryMetadataBuilder {
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

    /// Consumes the builder and constructs a [`BulkGeolocationLookupV2ResponseItemAbuseCountryMetadata`].
    /// This method will fail if any of the following fields are not set:
    /// - [`calling_code`](BulkGeolocationLookupV2ResponseItemAbuseCountryMetadataBuilder::calling_code)
    /// - [`tld`](BulkGeolocationLookupV2ResponseItemAbuseCountryMetadataBuilder::tld)
    /// - [`languages`](BulkGeolocationLookupV2ResponseItemAbuseCountryMetadataBuilder::languages)
    pub fn build(
        self,
    ) -> Result<BulkGeolocationLookupV2ResponseItemAbuseCountryMetadata, BuildError> {
        Ok(BulkGeolocationLookupV2ResponseItemAbuseCountryMetadata {
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
