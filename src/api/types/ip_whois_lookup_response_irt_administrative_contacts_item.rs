pub use crate::prelude::*;

/// Contact person or role object with handle, name, email, phone, and source reference.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IpWhoisLookupResponseIrtAdministrativeContactsItem {
    /// Handle or unique identifier of this contact in WHOIS data.
    #[serde(default)]
    pub handle: String,
    /// Name of the contact person or role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Address lines for this contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    /// Parsed street address from the address field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    /// Parsed city from the address field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Parsed district from the address field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    /// Parsed state from the address field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Parsed ZIP code from the address field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
    /// Parsed country from the address field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Email addresses for this contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Vec<String>>,
    /// Abuse mailbox email addresses for this contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abuse_mailbox: Option<Vec<String>>,
    /// Phone numbers for this contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Vec<String>>,
    /// Fax numbers for this contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fax_no: Option<Vec<String>>,
    /// Handles of organization objects managing this contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizations: Option<Vec<String>>,
    /// Handles of admin persons or roles linked to this contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_contacts: Option<Vec<String>>,
    /// Handles of technical persons or roles linked to this contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_contacts: Option<Vec<String>>,
    /// Remarks for this contact object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<Vec<String>>,
    /// Notify email addresses for this contact object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify: Option<Vec<String>>,
    /// Maintainer handles for this contact object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mnt_by: Option<Vec<String>>,
    /// Referenced maintainer handles for this contact object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mnt_ref: Option<Vec<String>>,
    /// Registration date of this contact object (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<NaiveDate>,
    /// Last update date of this contact object (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<NaiveDate>,
    /// WHOIS data source (RIR registry URL) for this object.
    #[serde(default)]
    pub source: String,
}

impl IpWhoisLookupResponseIrtAdministrativeContactsItem {
    pub fn builder() -> IpWhoisLookupResponseIrtAdministrativeContactsItemBuilder {
        <IpWhoisLookupResponseIrtAdministrativeContactsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IpWhoisLookupResponseIrtAdministrativeContactsItemBuilder {
    handle: Option<String>,
    name: Option<String>,
    address: Option<Vec<String>>,
    street: Option<String>,
    city: Option<String>,
    district: Option<String>,
    state: Option<String>,
    zip_code: Option<String>,
    country: Option<String>,
    email: Option<Vec<String>>,
    abuse_mailbox: Option<Vec<String>>,
    phone: Option<Vec<String>>,
    fax_no: Option<Vec<String>>,
    organizations: Option<Vec<String>>,
    admin_contacts: Option<Vec<String>>,
    tech_contacts: Option<Vec<String>>,
    remarks: Option<Vec<String>>,
    notify: Option<Vec<String>>,
    mnt_by: Option<Vec<String>>,
    mnt_ref: Option<Vec<String>>,
    date_created: Option<NaiveDate>,
    date_updated: Option<NaiveDate>,
    source: Option<String>,
}

impl IpWhoisLookupResponseIrtAdministrativeContactsItemBuilder {
    pub fn handle(mut self, value: impl Into<String>) -> Self {
        self.handle = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn address(mut self, value: Vec<String>) -> Self {
        self.address = Some(value);
        self
    }

    pub fn street(mut self, value: impl Into<String>) -> Self {
        self.street = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn district(mut self, value: impl Into<String>) -> Self {
        self.district = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn zip_code(mut self, value: impl Into<String>) -> Self {
        self.zip_code = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn email(mut self, value: Vec<String>) -> Self {
        self.email = Some(value);
        self
    }

    pub fn abuse_mailbox(mut self, value: Vec<String>) -> Self {
        self.abuse_mailbox = Some(value);
        self
    }

    pub fn phone(mut self, value: Vec<String>) -> Self {
        self.phone = Some(value);
        self
    }

    pub fn fax_no(mut self, value: Vec<String>) -> Self {
        self.fax_no = Some(value);
        self
    }

    pub fn organizations(mut self, value: Vec<String>) -> Self {
        self.organizations = Some(value);
        self
    }

    pub fn admin_contacts(mut self, value: Vec<String>) -> Self {
        self.admin_contacts = Some(value);
        self
    }

    pub fn tech_contacts(mut self, value: Vec<String>) -> Self {
        self.tech_contacts = Some(value);
        self
    }

    pub fn remarks(mut self, value: Vec<String>) -> Self {
        self.remarks = Some(value);
        self
    }

    pub fn notify(mut self, value: Vec<String>) -> Self {
        self.notify = Some(value);
        self
    }

    pub fn mnt_by(mut self, value: Vec<String>) -> Self {
        self.mnt_by = Some(value);
        self
    }

    pub fn mnt_ref(mut self, value: Vec<String>) -> Self {
        self.mnt_ref = Some(value);
        self
    }

    pub fn date_created(mut self, value: NaiveDate) -> Self {
        self.date_created = Some(value);
        self
    }

    pub fn date_updated(mut self, value: NaiveDate) -> Self {
        self.date_updated = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IpWhoisLookupResponseIrtAdministrativeContactsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`handle`](IpWhoisLookupResponseIrtAdministrativeContactsItemBuilder::handle)
    /// - [`source`](IpWhoisLookupResponseIrtAdministrativeContactsItemBuilder::source)
    pub fn build(self) -> Result<IpWhoisLookupResponseIrtAdministrativeContactsItem, BuildError> {
        Ok(IpWhoisLookupResponseIrtAdministrativeContactsItem {
            handle: self
                .handle
                .ok_or_else(|| BuildError::missing_field("handle"))?,
            name: self.name,
            address: self.address,
            street: self.street,
            city: self.city,
            district: self.district,
            state: self.state,
            zip_code: self.zip_code,
            country: self.country,
            email: self.email,
            abuse_mailbox: self.abuse_mailbox,
            phone: self.phone,
            fax_no: self.fax_no,
            organizations: self.organizations,
            admin_contacts: self.admin_contacts,
            tech_contacts: self.tech_contacts,
            remarks: self.remarks,
            notify: self.notify,
            mnt_by: self.mnt_by,
            mnt_ref: self.mnt_ref,
            date_created: self.date_created,
            date_updated: self.date_updated,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
