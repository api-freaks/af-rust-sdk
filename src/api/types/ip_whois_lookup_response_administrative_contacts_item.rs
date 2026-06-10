pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IpWhoisLookupResponseAdministrativeContactsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl IpWhoisLookupResponseAdministrativeContactsItem {
    pub fn builder() -> IpWhoisLookupResponseAdministrativeContactsItemBuilder {
        <IpWhoisLookupResponseAdministrativeContactsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IpWhoisLookupResponseAdministrativeContactsItemBuilder {
    handle: Option<String>,
    name: Option<String>,
    email: Option<Vec<String>>,
    phone: Option<Vec<String>>,
    source: Option<String>,
}

impl IpWhoisLookupResponseAdministrativeContactsItemBuilder {
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

    /// Consumes the builder and constructs a [`IpWhoisLookupResponseAdministrativeContactsItem`].
    pub fn build(self) -> Result<IpWhoisLookupResponseAdministrativeContactsItem, BuildError> {
        Ok(IpWhoisLookupResponseAdministrativeContactsItem {
            handle: self.handle,
            name: self.name,
            email: self.email,
            phone: self.phone,
            source: self.source,
        })
    }
}
