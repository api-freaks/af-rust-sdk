pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DomainWhoisHistoryResponseWhoisDomainsHistoricalItem {
    /// Shows the number of the record in the array.
    #[serde(default)]
    pub num: i64,
    /// Always true.
    #[serde(default)]
    pub status: bool,
    /// Domain name which was queried.
    #[serde(default)]
    pub domain_name: String,
    /// The timestamp when the query was made.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub query_time: DateTime<FixedOffset>,
    /// The WHOIS server that provided the domain information.
    #[serde(default)]
    pub whois_server: String,
    /// Domain registration status.
    pub domain_registered: DomainWhoisHistoryResponseWhoisDomainsHistoricalItemDomainRegistered,
    /// Date when the domain was initially registered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<NaiveDate>,
    /// The date of the most recent update to the domain registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<NaiveDate>,
    /// The date when the domain registration will expire if not renewed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_registrar:
        Option<DomainWhoisHistoryResponseWhoisDomainsHistoricalItemDomainRegistrar>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reseller_contact:
        Option<DomainWhoisHistoryResponseWhoisDomainsHistoricalItemResellerContact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_contact:
        Option<DomainWhoisHistoryResponseWhoisDomainsHistoricalItemRegistrantContact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_contact:
        Option<DomainWhoisHistoryResponseWhoisDomainsHistoricalItemAdministrativeContact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_contact:
        Option<DomainWhoisHistoryResponseWhoisDomainsHistoricalItemTechnicalContact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_contact: Option<DomainWhoisHistoryResponseWhoisDomainsHistoricalItemBillingContact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_servers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_status: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whois_raw_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_data: Option<DomainWhoisHistoryResponseWhoisDomainsHistoricalItemRegistryData>,
}

impl DomainWhoisHistoryResponseWhoisDomainsHistoricalItem {
    pub fn builder() -> DomainWhoisHistoryResponseWhoisDomainsHistoricalItemBuilder {
        <DomainWhoisHistoryResponseWhoisDomainsHistoricalItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainWhoisHistoryResponseWhoisDomainsHistoricalItemBuilder {
    num: Option<i64>,
    status: Option<bool>,
    domain_name: Option<String>,
    query_time: Option<DateTime<FixedOffset>>,
    whois_server: Option<String>,
    domain_registered: Option<DomainWhoisHistoryResponseWhoisDomainsHistoricalItemDomainRegistered>,
    create_date: Option<NaiveDate>,
    update_date: Option<NaiveDate>,
    expiry_date: Option<NaiveDate>,
    domain_registrar: Option<DomainWhoisHistoryResponseWhoisDomainsHistoricalItemDomainRegistrar>,
    reseller_contact: Option<DomainWhoisHistoryResponseWhoisDomainsHistoricalItemResellerContact>,
    registrant_contact:
        Option<DomainWhoisHistoryResponseWhoisDomainsHistoricalItemRegistrantContact>,
    administrative_contact:
        Option<DomainWhoisHistoryResponseWhoisDomainsHistoricalItemAdministrativeContact>,
    technical_contact: Option<DomainWhoisHistoryResponseWhoisDomainsHistoricalItemTechnicalContact>,
    billing_contact: Option<DomainWhoisHistoryResponseWhoisDomainsHistoricalItemBillingContact>,
    name_servers: Option<Vec<String>>,
    domain_status: Option<Vec<String>>,
    whois_raw_domain: Option<String>,
    registry_data: Option<DomainWhoisHistoryResponseWhoisDomainsHistoricalItemRegistryData>,
}

impl DomainWhoisHistoryResponseWhoisDomainsHistoricalItemBuilder {
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
        value: DomainWhoisHistoryResponseWhoisDomainsHistoricalItemDomainRegistered,
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
        value: DomainWhoisHistoryResponseWhoisDomainsHistoricalItemDomainRegistrar,
    ) -> Self {
        self.domain_registrar = Some(value);
        self
    }

    pub fn reseller_contact(
        mut self,
        value: DomainWhoisHistoryResponseWhoisDomainsHistoricalItemResellerContact,
    ) -> Self {
        self.reseller_contact = Some(value);
        self
    }

    pub fn registrant_contact(
        mut self,
        value: DomainWhoisHistoryResponseWhoisDomainsHistoricalItemRegistrantContact,
    ) -> Self {
        self.registrant_contact = Some(value);
        self
    }

    pub fn administrative_contact(
        mut self,
        value: DomainWhoisHistoryResponseWhoisDomainsHistoricalItemAdministrativeContact,
    ) -> Self {
        self.administrative_contact = Some(value);
        self
    }

    pub fn technical_contact(
        mut self,
        value: DomainWhoisHistoryResponseWhoisDomainsHistoricalItemTechnicalContact,
    ) -> Self {
        self.technical_contact = Some(value);
        self
    }

    pub fn billing_contact(
        mut self,
        value: DomainWhoisHistoryResponseWhoisDomainsHistoricalItemBillingContact,
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
        value: DomainWhoisHistoryResponseWhoisDomainsHistoricalItemRegistryData,
    ) -> Self {
        self.registry_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainWhoisHistoryResponseWhoisDomainsHistoricalItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`num`](DomainWhoisHistoryResponseWhoisDomainsHistoricalItemBuilder::num)
    /// - [`status`](DomainWhoisHistoryResponseWhoisDomainsHistoricalItemBuilder::status)
    /// - [`domain_name`](DomainWhoisHistoryResponseWhoisDomainsHistoricalItemBuilder::domain_name)
    /// - [`query_time`](DomainWhoisHistoryResponseWhoisDomainsHistoricalItemBuilder::query_time)
    /// - [`whois_server`](DomainWhoisHistoryResponseWhoisDomainsHistoricalItemBuilder::whois_server)
    /// - [`domain_registered`](DomainWhoisHistoryResponseWhoisDomainsHistoricalItemBuilder::domain_registered)
    pub fn build(self) -> Result<DomainWhoisHistoryResponseWhoisDomainsHistoricalItem, BuildError> {
        Ok(DomainWhoisHistoryResponseWhoisDomainsHistoricalItem {
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
        })
    }
}
