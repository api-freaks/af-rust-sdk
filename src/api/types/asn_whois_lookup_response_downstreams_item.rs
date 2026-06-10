pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AsnWhoisLookupResponseDownstreamsItem {
    #[serde(rename = "asNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl AsnWhoisLookupResponseDownstreamsItem {
    pub fn builder() -> AsnWhoisLookupResponseDownstreamsItemBuilder {
        <AsnWhoisLookupResponseDownstreamsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AsnWhoisLookupResponseDownstreamsItemBuilder {
    as_number: Option<String>,
    description: Option<String>,
    country: Option<String>,
}

impl AsnWhoisLookupResponseDownstreamsItemBuilder {
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

    /// Consumes the builder and constructs a [`AsnWhoisLookupResponseDownstreamsItem`].
    pub fn build(self) -> Result<AsnWhoisLookupResponseDownstreamsItem, BuildError> {
        Ok(AsnWhoisLookupResponseDownstreamsItem {
            as_number: self.as_number,
            description: self.description,
            country: self.country,
        })
    }
}
