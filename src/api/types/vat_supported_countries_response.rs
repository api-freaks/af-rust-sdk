pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatSupportedCountriesResponse {
    #[serde(rename = "VAT_Supported_Countries_And_States")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_supported_countries_and_states:
        Option<Vec<VatSupportedCountriesResponseVatSupportedCountriesAndStatesItem>>,
}

impl VatSupportedCountriesResponse {
    pub fn builder() -> VatSupportedCountriesResponseBuilder {
        <VatSupportedCountriesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatSupportedCountriesResponseBuilder {
    vat_supported_countries_and_states:
        Option<Vec<VatSupportedCountriesResponseVatSupportedCountriesAndStatesItem>>,
}

impl VatSupportedCountriesResponseBuilder {
    pub fn vat_supported_countries_and_states(
        mut self,
        value: Vec<VatSupportedCountriesResponseVatSupportedCountriesAndStatesItem>,
    ) -> Self {
        self.vat_supported_countries_and_states = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VatSupportedCountriesResponse`].
    pub fn build(self) -> Result<VatSupportedCountriesResponse, BuildError> {
        Ok(VatSupportedCountriesResponse {
            vat_supported_countries_and_states: self.vat_supported_countries_and_states,
        })
    }
}
