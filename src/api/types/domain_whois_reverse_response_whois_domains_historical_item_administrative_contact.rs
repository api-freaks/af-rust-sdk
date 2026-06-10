pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContact {
    #[serde(default)]
    pub num: i64,
    #[serde(default)]
    pub status: bool,
    #[serde(default)]
    pub domain_name: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub query_time: DateTime<FixedOffset>,
    #[serde(default)]
    pub whois_server: String,
    pub domain_registered: DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactDomainRegistered,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_registrar: Option<DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactDomainRegistrar>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reseller_contact: Option<DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactResellerContact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_contact: Option<DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactRegistrantContact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_contact: Option<DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactAdministrativeContact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_contact: Option<DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactTechnicalContact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_contact: Option<DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactBillingContact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_servers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_status: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whois_raw_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_data: Option<DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactRegistryData>,
}

impl DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContact {
    pub fn builder(
    ) -> DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactBuilder {
        <DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactBuilder {
    num: Option<i64>,
    status: Option<bool>,
    domain_name: Option<String>,
    query_time: Option<DateTime<FixedOffset>>,
    whois_server: Option<String>,
    domain_registered: Option<DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactDomainRegistered>,
    create_date: Option<NaiveDate>,
    update_date: Option<NaiveDate>,
    expiry_date: Option<NaiveDate>,
    domain_registrar: Option<DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactDomainRegistrar>,
    reseller_contact: Option<DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactResellerContact>,
    registrant_contact: Option<DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactRegistrantContact>,
    administrative_contact: Option<DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactAdministrativeContact>,
    technical_contact: Option<DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactTechnicalContact>,
    billing_contact: Option<DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactBillingContact>,
    name_servers: Option<Vec<String>>,
    domain_status: Option<Vec<String>>,
    whois_raw_domain: Option<String>,
    registry_data: Option<DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactRegistryData>,
}

impl DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactBuilder {
    pub fn num(mut self, value: i64) -> Self {
        self.num = Some(value);
        self
    }

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

    pub fn domain_registered(
        mut self,
        value: DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactDomainRegistered,
    ) -> Self {
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

    pub fn domain_registrar(
        mut self,
        value: DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactDomainRegistrar,
    ) -> Self {
        self.domain_registrar = Some(value);
        self
    }

    pub fn reseller_contact(
        mut self,
        value: DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactResellerContact,
    ) -> Self {
        self.reseller_contact = Some(value);
        self
    }

    pub fn registrant_contact(
        mut self,
        value: DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactRegistrantContact,
    ) -> Self {
        self.registrant_contact = Some(value);
        self
    }

    pub fn administrative_contact(
        mut self,
        value: DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactAdministrativeContact,
    ) -> Self {
        self.administrative_contact = Some(value);
        self
    }

    pub fn technical_contact(
        mut self,
        value: DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactTechnicalContact,
    ) -> Self {
        self.technical_contact = Some(value);
        self
    }

    pub fn billing_contact(
        mut self,
        value: DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactBillingContact,
    ) -> Self {
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

    pub fn registry_data(
        mut self,
        value: DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactRegistryData,
    ) -> Self {
        self.registry_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContact`].
    /// This method will fail if any of the following fields are not set:
    /// - [`num`](DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactBuilder::num)
    /// - [`status`](DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactBuilder::status)
    /// - [`domain_name`](DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactBuilder::domain_name)
    /// - [`query_time`](DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactBuilder::query_time)
    /// - [`whois_server`](DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactBuilder::whois_server)
    /// - [`domain_registered`](DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactBuilder::domain_registered)
    pub fn build(
        self,
    ) -> Result<DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContact, BuildError>
    {
        Ok(
            DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContact {
                num: self.num.ok_or_else(|| BuildError::missing_field("num"))?,
                status: self
                    .status
                    .ok_or_else(|| BuildError::missing_field("status"))?,
                domain_name: self
                    .domain_name
                    .ok_or_else(|| BuildError::missing_field("domain_name"))?,
                query_time: self
                    .query_time
                    .ok_or_else(|| BuildError::missing_field("query_time"))?,
                whois_server: self
                    .whois_server
                    .ok_or_else(|| BuildError::missing_field("whois_server"))?,
                domain_registered: self
                    .domain_registered
                    .ok_or_else(|| BuildError::missing_field("domain_registered"))?,
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
            },
        )
    }
}
