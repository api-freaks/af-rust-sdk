pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSupportedFlagsResponseSupportedCountriesItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub iso2: String,
    #[serde(default)]
    pub iso3: String,
}

impl GetSupportedFlagsResponseSupportedCountriesItem {
    pub fn builder() -> GetSupportedFlagsResponseSupportedCountriesItemBuilder {
        <GetSupportedFlagsResponseSupportedCountriesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSupportedFlagsResponseSupportedCountriesItemBuilder {
    name: Option<String>,
    iso2: Option<String>,
    iso3: Option<String>,
}

impl GetSupportedFlagsResponseSupportedCountriesItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn iso2(mut self, value: impl Into<String>) -> Self {
        self.iso2 = Some(value.into());
        self
    }

    pub fn iso3(mut self, value: impl Into<String>) -> Self {
        self.iso3 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetSupportedFlagsResponseSupportedCountriesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](GetSupportedFlagsResponseSupportedCountriesItemBuilder::name)
    /// - [`iso2`](GetSupportedFlagsResponseSupportedCountriesItemBuilder::iso2)
    /// - [`iso3`](GetSupportedFlagsResponseSupportedCountriesItemBuilder::iso3)
    pub fn build(self) -> Result<GetSupportedFlagsResponseSupportedCountriesItem, BuildError> {
        Ok(GetSupportedFlagsResponseSupportedCountriesItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            iso2: self.iso2.ok_or_else(|| BuildError::missing_field("iso2"))?,
            iso3: self.iso3.ok_or_else(|| BuildError::missing_field("iso3"))?,
        })
    }
}
