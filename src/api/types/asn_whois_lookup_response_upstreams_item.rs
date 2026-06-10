pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AsnWhoisLookupResponseUpstreamsItem {
    #[serde(rename = "asNumber")]
    #[serde(default)]
    pub as_number: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub country: String,
}

impl AsnWhoisLookupResponseUpstreamsItem {
    pub fn builder() -> AsnWhoisLookupResponseUpstreamsItemBuilder {
        <AsnWhoisLookupResponseUpstreamsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AsnWhoisLookupResponseUpstreamsItemBuilder {
    as_number: Option<String>,
    description: Option<String>,
    country: Option<String>,
}

impl AsnWhoisLookupResponseUpstreamsItemBuilder {
    pub fn as_number(mut self, value: impl Into<String>) -> Self {
        self.as_number = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AsnWhoisLookupResponseUpstreamsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`as_number`](AsnWhoisLookupResponseUpstreamsItemBuilder::as_number)
    /// - [`description`](AsnWhoisLookupResponseUpstreamsItemBuilder::description)
    /// - [`country`](AsnWhoisLookupResponseUpstreamsItemBuilder::country)
    pub fn build(self) -> Result<AsnWhoisLookupResponseUpstreamsItem, BuildError> {
        Ok(AsnWhoisLookupResponseUpstreamsItem {
            as_number: self
                .as_number
                .ok_or_else(|| BuildError::missing_field("as_number"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
        })
    }
}
