pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AsnWhoisLookupResponseParsedWhoisResponseAbuseContactsItem {
    #[serde(default)]
    pub handle: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub email: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl AsnWhoisLookupResponseParsedWhoisResponseAbuseContactsItem {
    pub fn builder() -> AsnWhoisLookupResponseParsedWhoisResponseAbuseContactsItemBuilder {
        <AsnWhoisLookupResponseParsedWhoisResponseAbuseContactsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AsnWhoisLookupResponseParsedWhoisResponseAbuseContactsItemBuilder {
    handle: Option<String>,
    name: Option<String>,
    email: Option<Vec<String>>,
    phone: Option<Vec<String>>,
    source: Option<String>,
}

impl AsnWhoisLookupResponseParsedWhoisResponseAbuseContactsItemBuilder {
    pub fn handle(mut self, value: impl Into<String>) -> Self {
        self.handle = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn email(mut self, value: Vec<String>) -> Self {
        self.email = Some(value);
        self
    }

    pub fn phone(mut self, value: Vec<String>) -> Self {
        self.phone = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AsnWhoisLookupResponseParsedWhoisResponseAbuseContactsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`handle`](AsnWhoisLookupResponseParsedWhoisResponseAbuseContactsItemBuilder::handle)
    /// - [`name`](AsnWhoisLookupResponseParsedWhoisResponseAbuseContactsItemBuilder::name)
    /// - [`email`](AsnWhoisLookupResponseParsedWhoisResponseAbuseContactsItemBuilder::email)
    pub fn build(
        self,
    ) -> Result<AsnWhoisLookupResponseParsedWhoisResponseAbuseContactsItem, BuildError> {
        Ok(AsnWhoisLookupResponseParsedWhoisResponseAbuseContactsItem {
            handle: self
                .handle
                .ok_or_else(|| BuildError::missing_field("handle"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            phone: self.phone,
            source: self.source,
        })
    }
}
