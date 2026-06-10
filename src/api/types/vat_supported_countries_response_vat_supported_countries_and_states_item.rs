pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatSupportedCountriesResponseVatSupportedCountriesAndStatesItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub states: Vec<String>,
}

impl VatSupportedCountriesResponseVatSupportedCountriesAndStatesItem {
    pub fn builder() -> VatSupportedCountriesResponseVatSupportedCountriesAndStatesItemBuilder {
        <VatSupportedCountriesResponseVatSupportedCountriesAndStatesItemBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatSupportedCountriesResponseVatSupportedCountriesAndStatesItemBuilder {
    code: Option<String>,
    name: Option<String>,
    states: Option<Vec<String>>,
}

impl VatSupportedCountriesResponseVatSupportedCountriesAndStatesItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn states(mut self, value: Vec<String>) -> Self {
        self.states = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VatSupportedCountriesResponseVatSupportedCountriesAndStatesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](VatSupportedCountriesResponseVatSupportedCountriesAndStatesItemBuilder::code)
    /// - [`name`](VatSupportedCountriesResponseVatSupportedCountriesAndStatesItemBuilder::name)
    /// - [`states`](VatSupportedCountriesResponseVatSupportedCountriesAndStatesItemBuilder::states)
    pub fn build(
        self,
    ) -> Result<VatSupportedCountriesResponseVatSupportedCountriesAndStatesItem, BuildError> {
        Ok(
            VatSupportedCountriesResponseVatSupportedCountriesAndStatesItem {
                code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
                name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
                states: self
                    .states
                    .ok_or_else(|| BuildError::missing_field("states"))?,
            },
        )
    }
}
