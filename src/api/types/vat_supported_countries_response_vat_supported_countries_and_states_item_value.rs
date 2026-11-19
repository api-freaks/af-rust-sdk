pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VatSupportedCountriesResponseVatSupportedCountriesAndStatesItemValue {
    /// ISO 3166-1 alpha-2 country code, lowercase.
    #[serde(default)]
    pub code: String,
    /// Sub-national regions with their own VAT data, keyed by name (e.g. `california`: `us:ca`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<HashMap<String, String>>,
}

impl VatSupportedCountriesResponseVatSupportedCountriesAndStatesItemValue {
    pub fn builder() -> VatSupportedCountriesResponseVatSupportedCountriesAndStatesItemValueBuilder
    {
        <VatSupportedCountriesResponseVatSupportedCountriesAndStatesItemValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatSupportedCountriesResponseVatSupportedCountriesAndStatesItemValueBuilder {
    code: Option<String>,
    states: Option<HashMap<String, String>>,
}

impl VatSupportedCountriesResponseVatSupportedCountriesAndStatesItemValueBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn states(mut self, value: HashMap<String, String>) -> Self {
        self.states = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VatSupportedCountriesResponseVatSupportedCountriesAndStatesItemValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](VatSupportedCountriesResponseVatSupportedCountriesAndStatesItemValueBuilder::code)
    pub fn build(
        self,
    ) -> Result<VatSupportedCountriesResponseVatSupportedCountriesAndStatesItemValue, BuildError>
    {
        Ok(
            VatSupportedCountriesResponseVatSupportedCountriesAndStatesItemValue {
                code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
                states: self.states,
            },
        )
    }
}
