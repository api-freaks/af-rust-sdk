pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSupportedFlagsResponseItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub iso2: String,
    #[serde(default)]
    pub iso3: String,
}

impl GetSupportedFlagsResponseItem {
    pub fn builder() -> GetSupportedFlagsResponseItemBuilder {
        <GetSupportedFlagsResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSupportedFlagsResponseItemBuilder {
    name: Option<String>,
    iso2: Option<String>,
    iso3: Option<String>,
}

impl GetSupportedFlagsResponseItemBuilder {
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

    /// Consumes the builder and constructs a [`GetSupportedFlagsResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](GetSupportedFlagsResponseItemBuilder::name)
    /// - [`iso2`](GetSupportedFlagsResponseItemBuilder::iso2)
    /// - [`iso3`](GetSupportedFlagsResponseItemBuilder::iso3)
    pub fn build(self) -> Result<GetSupportedFlagsResponseItem, BuildError> {
        Ok(GetSupportedFlagsResponseItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            iso2: self.iso2.ok_or_else(|| BuildError::missing_field("iso2"))?,
            iso3: self.iso3.ok_or_else(|| BuildError::missing_field("iso3"))?,
        })
    }
}
