pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IpWhoisLookupResponseTechnicalContactsItem {
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
    /// Abuse mailbox email addresses for this contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abuse_mailbox: Option<Vec<String>>,
    /// Address lines for this contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    /// Handles of admin persons or roles linked to this contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_contacts: Option<Vec<String>>,
    /// Parsed city from the address field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Parsed country from the address field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Registration date of this contact object (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<NaiveDate>,
    /// Last update date of this contact object (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<NaiveDate>,
    /// Parsed district from the address field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    /// Fax numbers for this contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fax_no: Option<Vec<String>>,
    /// Maintainer handles for this contact object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mnt_by: Option<Vec<String>>,
    /// Referenced maintainer handles for this contact object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mnt_ref: Option<Vec<String>>,
    /// Notify email addresses for this contact object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify: Option<Vec<String>>,
    /// Handles of organization objects managing this contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizations: Option<Vec<String>>,
    /// Remarks for this contact object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<Vec<String>>,
    /// Parsed state from the address field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Parsed street address from the address field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    /// Handles of technical persons or roles linked to this contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_contacts: Option<Vec<String>>,
    /// Parsed ZIP code from the address field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
}

impl IpWhoisLookupResponseTechnicalContactsItem {
    pub fn builder() -> IpWhoisLookupResponseTechnicalContactsItemBuilder {
        <IpWhoisLookupResponseTechnicalContactsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IpWhoisLookupResponseTechnicalContactsItemBuilder {
    handle: Option<String>,
    name: Option<String>,
    email: Option<Vec<String>>,
    phone: Option<Vec<String>>,
    source: Option<String>,
    abuse_mailbox: Option<Vec<String>>,
    address: Option<Vec<String>>,
    admin_contacts: Option<Vec<String>>,
    city: Option<String>,
    country: Option<String>,
    date_created: Option<NaiveDate>,
    date_updated: Option<NaiveDate>,
    district: Option<String>,
    fax_no: Option<Vec<String>>,
    mnt_by: Option<Vec<String>>,
    mnt_ref: Option<Vec<String>>,
    notify: Option<Vec<String>>,
    organizations: Option<Vec<String>>,
    remarks: Option<Vec<String>>,
    state: Option<String>,
    street: Option<String>,
    tech_contacts: Option<Vec<String>>,
    zip_code: Option<String>,
}

impl IpWhoisLookupResponseTechnicalContactsItemBuilder {
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

    pub fn abuse_mailbox(mut self, value: Vec<String>) -> Self {
        self.abuse_mailbox = Some(value);
        self
    }

    pub fn address(mut self, value: Vec<String>) -> Self {
        self.address = Some(value);
        self
    }

    pub fn admin_contacts(mut self, value: Vec<String>) -> Self {
        self.admin_contacts = Some(value);
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
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

    pub fn district(mut self, value: impl Into<String>) -> Self {
        self.district = Some(value.into());
        self
    }

    pub fn fax_no(mut self, value: Vec<String>) -> Self {
        self.fax_no = Some(value);
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

    pub fn notify(mut self, value: Vec<String>) -> Self {
        self.notify = Some(value);
        self
    }

    pub fn organizations(mut self, value: Vec<String>) -> Self {
        self.organizations = Some(value);
        self
    }

    pub fn remarks(mut self, value: Vec<String>) -> Self {
        self.remarks = Some(value);
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn street(mut self, value: impl Into<String>) -> Self {
        self.street = Some(value.into());
        self
    }

    pub fn tech_contacts(mut self, value: Vec<String>) -> Self {
        self.tech_contacts = Some(value);
        self
    }

    pub fn zip_code(mut self, value: impl Into<String>) -> Self {
        self.zip_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IpWhoisLookupResponseTechnicalContactsItem`].
    pub fn build(self) -> Result<IpWhoisLookupResponseTechnicalContactsItem, BuildError> {
        Ok(IpWhoisLookupResponseTechnicalContactsItem {
            handle: self.handle,
            name: self.name,
            email: self.email,
            phone: self.phone,
            source: self.source,
            abuse_mailbox: self.abuse_mailbox,
            address: self.address,
            admin_contacts: self.admin_contacts,
            city: self.city,
            country: self.country,
            date_created: self.date_created,
            date_updated: self.date_updated,
            district: self.district,
            fax_no: self.fax_no,
            mnt_by: self.mnt_by,
            mnt_ref: self.mnt_ref,
            notify: self.notify,
            organizations: self.organizations,
            remarks: self.remarks,
            state: self.state,
            street: self.street,
            tech_contacts: self.tech_contacts,
            zip_code: self.zip_code,
        })
    }
}
