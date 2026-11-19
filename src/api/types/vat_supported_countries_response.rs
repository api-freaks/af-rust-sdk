pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VatSupportedCountriesResponse {
    #[serde(rename = "VAT_Supported_Countries_And_States")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_supported_countries_and_states: Option<
        Vec<HashMap<String, VatSupportedCountriesResponseVatSupportedCountriesAndStatesItemValue>>,
    >,
    /// Array of countries supported by the IBAN Validation API.
    #[serde(rename = "IBAN_Supported_Countries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban_supported_countries:
        Option<Vec<VatSupportedCountriesResponseIbanSupportedCountriesItem>>,
    /// Array of countries supported by the SWIFT/BIC Code Lookup and Finder APIs.
    #[serde(rename = "SWIFT_Supported_Countries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swift_supported_countries:
        Option<Vec<VatSupportedCountriesResponseSwiftSupportedCountriesItem>>,
}

impl VatSupportedCountriesResponse {
    pub fn builder() -> VatSupportedCountriesResponseBuilder {
        <VatSupportedCountriesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatSupportedCountriesResponseBuilder {
    vat_supported_countries_and_states: Option<
        Vec<HashMap<String, VatSupportedCountriesResponseVatSupportedCountriesAndStatesItemValue>>,
    >,
    iban_supported_countries: Option<Vec<VatSupportedCountriesResponseIbanSupportedCountriesItem>>,
    swift_supported_countries:
        Option<Vec<VatSupportedCountriesResponseSwiftSupportedCountriesItem>>,
}

impl VatSupportedCountriesResponseBuilder {
    pub fn vat_supported_countries_and_states(
        mut self,
        value: Vec<
            HashMap<String, VatSupportedCountriesResponseVatSupportedCountriesAndStatesItemValue>,
        >,
    ) -> Self {
        self.vat_supported_countries_and_states = Some(value);
        self
    }

    pub fn iban_supported_countries(
        mut self,
        value: Vec<VatSupportedCountriesResponseIbanSupportedCountriesItem>,
    ) -> Self {
        self.iban_supported_countries = Some(value);
        self
    }

    pub fn swift_supported_countries(
        mut self,
        value: Vec<VatSupportedCountriesResponseSwiftSupportedCountriesItem>,
    ) -> Self {
        self.swift_supported_countries = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VatSupportedCountriesResponse`].
    pub fn build(self) -> Result<VatSupportedCountriesResponse, BuildError> {
        Ok(VatSupportedCountriesResponse {
            vat_supported_countries_and_states: self.vat_supported_countries_and_states,
            iban_supported_countries: self.iban_supported_countries,
            swift_supported_countries: self.swift_supported_countries,
        })
    }
}
