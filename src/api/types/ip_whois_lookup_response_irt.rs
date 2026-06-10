pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IpWhoisLookupResponseIrt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abuse_mailbox: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fax_no: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizations: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_contacts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_contacts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub irt_nfy: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mnt_by: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mnt_ref: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl IpWhoisLookupResponseIrt {
    pub fn builder() -> IpWhoisLookupResponseIrtBuilder {
        <IpWhoisLookupResponseIrtBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IpWhoisLookupResponseIrtBuilder {
    handle: Option<String>,
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
    signature: Option<Vec<String>>,
    encryption: Option<Vec<String>>,
    auth: Option<Vec<String>>,
    notify: Option<Vec<String>>,
    irt_nfy: Option<Vec<String>>,
    mnt_by: Option<Vec<String>>,
    mnt_ref: Option<Vec<String>>,
    date_created: Option<String>,
    date_updated: Option<String>,
    source: Option<String>,
}

impl IpWhoisLookupResponseIrtBuilder {
    pub fn handle(mut self, value: impl Into<String>) -> Self {
        self.handle = Some(value.into());
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

    pub fn signature(mut self, value: Vec<String>) -> Self {
        self.signature = Some(value);
        self
    }

    pub fn encryption(mut self, value: Vec<String>) -> Self {
        self.encryption = Some(value);
        self
    }

    pub fn auth(mut self, value: Vec<String>) -> Self {
        self.auth = Some(value);
        self
    }

    pub fn notify(mut self, value: Vec<String>) -> Self {
        self.notify = Some(value);
        self
    }

    pub fn irt_nfy(mut self, value: Vec<String>) -> Self {
        self.irt_nfy = Some(value);
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

    pub fn date_created(mut self, value: impl Into<String>) -> Self {
        self.date_created = Some(value.into());
        self
    }

    pub fn date_updated(mut self, value: impl Into<String>) -> Self {
        self.date_updated = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IpWhoisLookupResponseIrt`].
    pub fn build(self) -> Result<IpWhoisLookupResponseIrt, BuildError> {
        Ok(IpWhoisLookupResponseIrt {
            handle: self.handle,
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
            signature: self.signature,
            encryption: self.encryption,
            auth: self.auth,
            notify: self.notify,
            irt_nfy: self.irt_nfy,
            mnt_by: self.mnt_by,
            mnt_ref: self.mnt_ref,
            date_created: self.date_created,
            date_updated: self.date_updated,
            source: self.source,
        })
    }
}
