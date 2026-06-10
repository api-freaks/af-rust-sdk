pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct IpWhoisLookupResponseInetNumsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub countries: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geofeed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sponsoring_organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_contacts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_contacts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abuse_contacts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mnt_by: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mnt_lower: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mnt_domains: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mnt_routes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mnt_irt: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<String>>,
}

impl IpWhoisLookupResponseInetNumsItem {
    pub fn builder() -> IpWhoisLookupResponseInetNumsItemBuilder {
        <IpWhoisLookupResponseInetNumsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IpWhoisLookupResponseInetNumsItemBuilder {
    start_ip: Option<String>,
    end_ip: Option<String>,
    cidr: Option<Vec<String>>,
    net_name: Option<String>,
    net_handle: Option<String>,
    description: Option<Vec<String>>,
    countries: Option<Vec<String>>,
    geofeed: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    city: Option<String>,
    languages: Option<Vec<String>>,
    status: Option<String>,
    organization: Option<String>,
    sponsoring_organization: Option<String>,
    admin_contacts: Option<Vec<String>>,
    tech_contacts: Option<Vec<String>>,
    abuse_contacts: Option<Vec<String>>,
    remarks: Option<Vec<String>>,
    assignment_size: Option<String>,
    notify: Option<Vec<String>>,
    mnt_by: Option<Vec<String>>,
    mnt_lower: Option<Vec<String>>,
    mnt_domains: Option<Vec<String>>,
    mnt_routes: Option<Vec<String>>,
    mnt_irt: Option<Vec<String>>,
    date_created: Option<String>,
    date_updated: Option<String>,
    source: Option<String>,
    parents: Option<Vec<String>>,
}

impl IpWhoisLookupResponseInetNumsItemBuilder {
    pub fn start_ip(mut self, value: impl Into<String>) -> Self {
        self.start_ip = Some(value.into());
        self
    }

    pub fn end_ip(mut self, value: impl Into<String>) -> Self {
        self.end_ip = Some(value.into());
        self
    }

    pub fn cidr(mut self, value: Vec<String>) -> Self {
        self.cidr = Some(value);
        self
    }

    pub fn net_name(mut self, value: impl Into<String>) -> Self {
        self.net_name = Some(value.into());
        self
    }

    pub fn net_handle(mut self, value: impl Into<String>) -> Self {
        self.net_handle = Some(value.into());
        self
    }

    pub fn description(mut self, value: Vec<String>) -> Self {
        self.description = Some(value);
        self
    }

    pub fn countries(mut self, value: Vec<String>) -> Self {
        self.countries = Some(value);
        self
    }

    pub fn geofeed(mut self, value: impl Into<String>) -> Self {
        self.geofeed = Some(value.into());
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

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn languages(mut self, value: Vec<String>) -> Self {
        self.languages = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn organization(mut self, value: impl Into<String>) -> Self {
        self.organization = Some(value.into());
        self
    }

    pub fn sponsoring_organization(mut self, value: impl Into<String>) -> Self {
        self.sponsoring_organization = Some(value.into());
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

    pub fn assignment_size(mut self, value: impl Into<String>) -> Self {
        self.assignment_size = Some(value.into());
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

    pub fn mnt_lower(mut self, value: Vec<String>) -> Self {
        self.mnt_lower = Some(value);
        self
    }

    pub fn mnt_domains(mut self, value: Vec<String>) -> Self {
        self.mnt_domains = Some(value);
        self
    }

    pub fn mnt_routes(mut self, value: Vec<String>) -> Self {
        self.mnt_routes = Some(value);
        self
    }

    pub fn mnt_irt(mut self, value: Vec<String>) -> Self {
        self.mnt_irt = Some(value);
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

    pub fn parents(mut self, value: Vec<String>) -> Self {
        self.parents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IpWhoisLookupResponseInetNumsItem`].
    pub fn build(self) -> Result<IpWhoisLookupResponseInetNumsItem, BuildError> {
        Ok(IpWhoisLookupResponseInetNumsItem {
            start_ip: self.start_ip,
            end_ip: self.end_ip,
            cidr: self.cidr,
            net_name: self.net_name,
            net_handle: self.net_handle,
            description: self.description,
            countries: self.countries,
            geofeed: self.geofeed,
            latitude: self.latitude,
            longitude: self.longitude,
            city: self.city,
            languages: self.languages,
            status: self.status,
            organization: self.organization,
            sponsoring_organization: self.sponsoring_organization,
            admin_contacts: self.admin_contacts,
            tech_contacts: self.tech_contacts,
            abuse_contacts: self.abuse_contacts,
            remarks: self.remarks,
            assignment_size: self.assignment_size,
            notify: self.notify,
            mnt_by: self.mnt_by,
            mnt_lower: self.mnt_lower,
            mnt_domains: self.mnt_domains,
            mnt_routes: self.mnt_routes,
            mnt_irt: self.mnt_irt,
            date_created: self.date_created,
            date_updated: self.date_updated,
            source: self.source,
            parents: self.parents,
        })
    }
}
