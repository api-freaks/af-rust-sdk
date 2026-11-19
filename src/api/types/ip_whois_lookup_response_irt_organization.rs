pub use crate::prelude::*;

/// Organization object containing the registrant's handle, name, address, contact details, and linked admin/tech/abuse contacts.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct IpWhoisLookupResponseIrtOrganization {
    /// Handle or unique identifier of this organization in WHOIS data.
    #[serde(default)]
    pub handle: String,
    /// Name of the organization.
    #[serde(default)]
    pub name: String,
    /// Type of the organization (e.g., BUSINESS, REGISTRY).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Description values from the organization object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    /// Full address lines of the organization.
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
    /// ISO 3166-1 alpha-2 country codes for the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    /// Latitude parsed from the geoloc field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    /// Longitude parsed from the geoloc field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    /// Email addresses for the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Vec<String>>,
    /// Abuse mailbox email addresses for the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abuse_mailbox: Option<Vec<String>>,
    /// Phone numbers for the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Vec<String>>,
    /// Fax numbers for the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fax_no: Option<Vec<String>>,
    /// Handles of admin persons or roles linked to this organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_contacts: Option<Vec<String>>,
    /// Handles of technical persons or roles linked to this organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_contacts: Option<Vec<String>>,
    /// Handles of abuse persons or roles linked to this organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abuse_contacts: Option<Vec<String>>,
    /// Remarks for this organization object.
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
    #[serde(default)]
    pub source: String,
}

impl IpWhoisLookupResponseIrtOrganization {
    pub fn builder() -> IpWhoisLookupResponseIrtOrganizationBuilder {
        <IpWhoisLookupResponseIrtOrganizationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IpWhoisLookupResponseIrtOrganizationBuilder {
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
    admin_contacts: Option<Vec<String>>,
    tech_contacts: Option<Vec<String>>,
    abuse_contacts: Option<Vec<String>>,
    remarks: Option<Vec<String>>,
    notify: Option<Vec<String>>,
    ref_nfy: Option<Vec<String>>,
    mnt_by: Option<Vec<String>>,
    mnt_ref: Option<Vec<String>>,
    date_created: Option<NaiveDate>,
    date_updated: Option<NaiveDate>,
    source: Option<String>,
}

impl IpWhoisLookupResponseIrtOrganizationBuilder {
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

    /// Consumes the builder and constructs a [`IpWhoisLookupResponseIrtOrganization`].
    /// This method will fail if any of the following fields are not set:
    /// - [`handle`](IpWhoisLookupResponseIrtOrganizationBuilder::handle)
    /// - [`name`](IpWhoisLookupResponseIrtOrganizationBuilder::name)
    /// - [`source`](IpWhoisLookupResponseIrtOrganizationBuilder::source)
    pub fn build(self) -> Result<IpWhoisLookupResponseIrtOrganization, BuildError> {
        Ok(IpWhoisLookupResponseIrtOrganization {
            handle: self
                .handle
                .ok_or_else(|| BuildError::missing_field("handle"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
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
            admin_contacts: self.admin_contacts,
            tech_contacts: self.tech_contacts,
            abuse_contacts: self.abuse_contacts,
            remarks: self.remarks,
            notify: self.notify,
            ref_nfy: self.ref_nfy,
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
