pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainWhoisLookupResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub query_time: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whois_server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_registered: Option<DomainWhoisLookupResponseDomainRegistered>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_registrar: Option<DomainWhoisLookupResponseDomainRegistrar>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reseller_contact: Option<DomainWhoisLookupResponseResellerContact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_contact: Option<DomainWhoisLookupResponseRegistrantContact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_contact: Option<DomainWhoisLookupResponseAdministrativeContact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_contact: Option<DomainWhoisLookupResponseTechnicalContact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_contact: Option<DomainWhoisLookupResponseBillingContact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_servers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_status: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whois_raw_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_data: Option<DomainWhoisLookupResponseRegistryData>,
}

impl DomainWhoisLookupResponse {
    pub fn builder() -> DomainWhoisLookupResponseBuilder {
        <DomainWhoisLookupResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainWhoisLookupResponseBuilder {
    status: Option<bool>,
    domain_name: Option<String>,
    query_time: Option<DateTime<FixedOffset>>,
    whois_server: Option<String>,
    domain_registered: Option<DomainWhoisLookupResponseDomainRegistered>,
    create_date: Option<NaiveDate>,
    update_date: Option<NaiveDate>,
    expiry_date: Option<NaiveDate>,
    domain_registrar: Option<DomainWhoisLookupResponseDomainRegistrar>,
    reseller_contact: Option<DomainWhoisLookupResponseResellerContact>,
    registrant_contact: Option<DomainWhoisLookupResponseRegistrantContact>,
    administrative_contact: Option<DomainWhoisLookupResponseAdministrativeContact>,
    technical_contact: Option<DomainWhoisLookupResponseTechnicalContact>,
    billing_contact: Option<DomainWhoisLookupResponseBillingContact>,
    name_servers: Option<Vec<String>>,
    domain_status: Option<Vec<String>>,
    whois_raw_domain: Option<String>,
    registry_data: Option<DomainWhoisLookupResponseRegistryData>,
}

impl DomainWhoisLookupResponseBuilder {
    pub fn status(mut self, value: bool) -> Self {
        self.status = Some(value);
        self
    }

    pub fn domain_name(mut self, value: impl Into<String>) -> Self {
        self.domain_name = Some(value.into());
        self
    }

    pub fn query_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.query_time = Some(value);
        self
    }

    pub fn whois_server(mut self, value: impl Into<String>) -> Self {
        self.whois_server = Some(value.into());
        self
    }

    pub fn domain_registered(mut self, value: DomainWhoisLookupResponseDomainRegistered) -> Self {
        self.domain_registered = Some(value);
        self
    }

    pub fn create_date(mut self, value: NaiveDate) -> Self {
        self.create_date = Some(value);
        self
    }

    pub fn update_date(mut self, value: NaiveDate) -> Self {
        self.update_date = Some(value);
        self
    }

    pub fn expiry_date(mut self, value: NaiveDate) -> Self {
        self.expiry_date = Some(value);
        self
    }

    pub fn domain_registrar(mut self, value: DomainWhoisLookupResponseDomainRegistrar) -> Self {
        self.domain_registrar = Some(value);
        self
    }

    pub fn reseller_contact(mut self, value: DomainWhoisLookupResponseResellerContact) -> Self {
        self.reseller_contact = Some(value);
        self
    }

    pub fn registrant_contact(mut self, value: DomainWhoisLookupResponseRegistrantContact) -> Self {
        self.registrant_contact = Some(value);
        self
    }

    pub fn administrative_contact(
        mut self,
        value: DomainWhoisLookupResponseAdministrativeContact,
    ) -> Self {
        self.administrative_contact = Some(value);
        self
    }

    pub fn technical_contact(mut self, value: DomainWhoisLookupResponseTechnicalContact) -> Self {
        self.technical_contact = Some(value);
        self
    }

    pub fn billing_contact(mut self, value: DomainWhoisLookupResponseBillingContact) -> Self {
        self.billing_contact = Some(value);
        self
    }

    pub fn name_servers(mut self, value: Vec<String>) -> Self {
        self.name_servers = Some(value);
        self
    }

    pub fn domain_status(mut self, value: Vec<String>) -> Self {
        self.domain_status = Some(value);
        self
    }

    pub fn whois_raw_domain(mut self, value: impl Into<String>) -> Self {
        self.whois_raw_domain = Some(value.into());
        self
    }

    pub fn registry_data(mut self, value: DomainWhoisLookupResponseRegistryData) -> Self {
        self.registry_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainWhoisLookupResponse`].
    pub fn build(self) -> Result<DomainWhoisLookupResponse, BuildError> {
        Ok(DomainWhoisLookupResponse {
            status: self.status,
            domain_name: self.domain_name,
            query_time: self.query_time,
            whois_server: self.whois_server,
            domain_registered: self.domain_registered,
            create_date: self.create_date,
            update_date: self.update_date,
            expiry_date: self.expiry_date,
            domain_registrar: self.domain_registrar,
            reseller_contact: self.reseller_contact,
            registrant_contact: self.registrant_contact,
            administrative_contact: self.administrative_contact,
            technical_contact: self.technical_contact,
            billing_contact: self.billing_contact,
            name_servers: self.name_servers,
            domain_status: self.domain_status,
            whois_raw_domain: self.whois_raw_domain,
            registry_data: self.registry_data,
        })
    }
}
