pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AsnWhoisLookupResponseContacts {
    #[serde(rename = "emailContacts")]
    #[serde(default)]
    pub email_contacts: Vec<String>,
    #[serde(rename = "abuseContacts")]
    #[serde(default)]
    pub abuse_contacts: Vec<String>,
}

impl AsnWhoisLookupResponseContacts {
    pub fn builder() -> AsnWhoisLookupResponseContactsBuilder {
        <AsnWhoisLookupResponseContactsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AsnWhoisLookupResponseContactsBuilder {
    email_contacts: Option<Vec<String>>,
    abuse_contacts: Option<Vec<String>>,
}

impl AsnWhoisLookupResponseContactsBuilder {
    pub fn email_contacts(mut self, value: Vec<String>) -> Self {
        self.email_contacts = Some(value);
        self
    }

    pub fn abuse_contacts(mut self, value: Vec<String>) -> Self {
        self.abuse_contacts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AsnWhoisLookupResponseContacts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email_contacts`](AsnWhoisLookupResponseContactsBuilder::email_contacts)
    /// - [`abuse_contacts`](AsnWhoisLookupResponseContactsBuilder::abuse_contacts)
    pub fn build(self) -> Result<AsnWhoisLookupResponseContacts, BuildError> {
        Ok(AsnWhoisLookupResponseContacts {
            email_contacts: self
                .email_contacts
                .ok_or_else(|| BuildError::missing_field("email_contacts"))?,
            abuse_contacts: self
                .abuse_contacts
                .ok_or_else(|| BuildError::missing_field("abuse_contacts"))?,
        })
    }
}
