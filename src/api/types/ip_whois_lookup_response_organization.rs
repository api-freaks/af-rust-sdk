pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct IpWhoisLookupResponseOrganization {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
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
    pub country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub longitude: Option<f64>,
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
    pub abuse_contacts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<Vec<String>>,
    /// Notify email addresses for this organization object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify: Option<Vec<String>>,
    /// Reference notification email addresses for this organization object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_nfy: Option<Vec<String>>,
    /// Maintainer handles for this organization object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mnt_by: Option<Vec<String>>,
    /// Referenced maintainer handles for this organization object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mnt_ref: Option<Vec<String>>,
    /// Registration date of this organization object (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<NaiveDate>,
    /// Last update date of this organization object (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<NaiveDate>,
    /// WHOIS data source (RIR registry URL) for this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl IpWhoisLookupResponseOrganization {
    pub fn builder() -> IpWhoisLookupResponseOrganizationBuilder {
        <IpWhoisLookupResponseOrganizationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IpWhoisLookupResponseOrganizationBuilder {
    handle: Option<String>,
    name: Option<String>,
    r#type: Option<String>,
    description: Option<Vec<String>>,
    address: Option<Vec<String>>,
    street: Option<String>,
    city: Option<String>,
    district: Option<String>,
    state: Option<String>,
    zip_code: Option<String>,
    country: Option<Vec<String>>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    email: Option<Vec<String>>,
    abuse_mailbox: Option<Vec<String>>,
    phone: Option<Vec<String>>,
    fax_no: Option<Vec<String>>,
    organizations: Option<Vec<String>>,
    admin_contacts: Option<Vec<String>>,
    tech_contacts: Option<Vec<String>>,
    abuse_contacts: Option<Vec<String>>,
    languages: Option<Vec<String>>,
    remarks: Option<Vec<String>>,
    notify: Option<Vec<String>>,
    ref_nfy: Option<Vec<String>>,
    mnt_by: Option<Vec<String>>,
    mnt_ref: Option<Vec<String>>,
    date_created: Option<NaiveDate>,
    date_updated: Option<NaiveDate>,
    source: Option<String>,
}

impl IpWhoisLookupResponseOrganizationBuilder {
    pub fn handle(mut self, value: impl Into<String>) -> Self {
        self.handle = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn description(mut self, value: Vec<String>) -> Self {
        self.description = Some(value);
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

    pub fn country(mut self, value: Vec<String>) -> Self {
        self.country = Some(value);
        self
    }

    pub fn latitude(mut self, value: f64) -> Self {
        self.latitude = Some(value);
        self
    }

    pub fn longitude(mut self, value: f64) -> Self {
        self.longitude = Some(value);
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

    pub fn abuse_contacts(mut self, value: Vec<String>) -> Self {
        self.abuse_contacts = Some(value);
        self
    }

    pub fn languages(mut self, value: Vec<String>) -> Self {
        self.languages = Some(value);
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

    pub fn ref_nfy(mut self, value: Vec<String>) -> Self {
        self.ref_nfy = Some(value);
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

    /// Consumes the builder and constructs a [`IpWhoisLookupResponseOrganization`].
    pub fn build(self) -> Result<IpWhoisLookupResponseOrganization, BuildError> {
        Ok(IpWhoisLookupResponseOrganization {
            handle: self.handle,
            name: self.name,
            r#type: self.r#type,
            description: self.description,
            address: self.address,
            street: self.street,
            city: self.city,
            district: self.district,
            state: self.state,
            zip_code: self.zip_code,
            country: self.country,
            latitude: self.latitude,
            longitude: self.longitude,
            email: self.email,
            abuse_mailbox: self.abuse_mailbox,
            phone: self.phone,
            fax_no: self.fax_no,
            organizations: self.organizations,
            admin_contacts: self.admin_contacts,
            tech_contacts: self.tech_contacts,
            abuse_contacts: self.abuse_contacts,
            languages: self.languages,
            remarks: self.remarks,
            notify: self.notify,
            ref_nfy: self.ref_nfy,
            mnt_by: self.mnt_by,
            mnt_ref: self.mnt_ref,
            date_created: self.date_created,
            date_updated: self.date_updated,
            source: self.source,
        })
    }
}
