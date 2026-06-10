pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AsnWhoisLookupResponsePeersItem {
    #[serde(rename = "asNumber")]
    #[serde(default)]
    pub as_number: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub country: String,
}

impl AsnWhoisLookupResponsePeersItem {
    pub fn builder() -> AsnWhoisLookupResponsePeersItemBuilder {
        <AsnWhoisLookupResponsePeersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AsnWhoisLookupResponsePeersItemBuilder {
    as_number: Option<String>,
    description: Option<String>,
    country: Option<String>,
}

impl AsnWhoisLookupResponsePeersItemBuilder {
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

    /// Consumes the builder and constructs a [`AsnWhoisLookupResponsePeersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`as_number`](AsnWhoisLookupResponsePeersItemBuilder::as_number)
    /// - [`description`](AsnWhoisLookupResponsePeersItemBuilder::description)
    /// - [`country`](AsnWhoisLookupResponsePeersItemBuilder::country)
    pub fn build(self) -> Result<AsnWhoisLookupResponsePeersItem, BuildError> {
        Ok(AsnWhoisLookupResponsePeersItem {
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
