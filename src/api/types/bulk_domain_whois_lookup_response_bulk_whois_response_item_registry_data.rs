pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkDomainWhoisLookupResponseBulkWhoisResponseItemRegistryData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub query_time: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whois_server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_registered:
        Option<BulkDomainWhoisLookupResponseBulkWhoisResponseItemRegistryDataDomainRegistered>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_servers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_status: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whois_raw_registry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_registrar:
        Option<BulkDomainWhoisLookupResponseBulkWhoisResponseItemRegistryDataDomainRegistrar>,
}

impl BulkDomainWhoisLookupResponseBulkWhoisResponseItemRegistryData {
    pub fn builder() -> BulkDomainWhoisLookupResponseBulkWhoisResponseItemRegistryDataBuilder {
        <BulkDomainWhoisLookupResponseBulkWhoisResponseItemRegistryDataBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDomainWhoisLookupResponseBulkWhoisResponseItemRegistryDataBuilder {
    domain_name: Option<String>,
    query_time: Option<DateTime<FixedOffset>>,
    whois_server: Option<String>,
    domain_registered:
        Option<BulkDomainWhoisLookupResponseBulkWhoisResponseItemRegistryDataDomainRegistered>,
    create_date: Option<NaiveDate>,
    update_date: Option<NaiveDate>,
    expiry_date: Option<NaiveDate>,
    name_servers: Option<Vec<String>>,
    domain_status: Option<Vec<String>>,
    whois_raw_registry: Option<String>,
    domain_registrar:
        Option<BulkDomainWhoisLookupResponseBulkWhoisResponseItemRegistryDataDomainRegistrar>,
}

impl BulkDomainWhoisLookupResponseBulkWhoisResponseItemRegistryDataBuilder {
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
        value: BulkDomainWhoisLookupResponseBulkWhoisResponseItemRegistryDataDomainRegistered,
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

    pub fn name_servers(mut self, value: Vec<String>) -> Self {
        self.name_servers = Some(value);
        self
    }

    pub fn domain_status(mut self, value: Vec<String>) -> Self {
        self.domain_status = Some(value);
        self
    }

    pub fn whois_raw_registry(mut self, value: impl Into<String>) -> Self {
        self.whois_raw_registry = Some(value.into());
        self
    }

    pub fn domain_registrar(
        mut self,
        value: BulkDomainWhoisLookupResponseBulkWhoisResponseItemRegistryDataDomainRegistrar,
    ) -> Self {
        self.domain_registrar = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkDomainWhoisLookupResponseBulkWhoisResponseItemRegistryData`].
    pub fn build(
        self,
    ) -> Result<BulkDomainWhoisLookupResponseBulkWhoisResponseItemRegistryData, BuildError> {
        Ok(
            BulkDomainWhoisLookupResponseBulkWhoisResponseItemRegistryData {
                domain_name: self.domain_name,
                query_time: self.query_time,
                whois_server: self.whois_server,
                domain_registered: self.domain_registered,
                create_date: self.create_date,
                update_date: self.update_date,
                expiry_date: self.expiry_date,
                name_servers: self.name_servers,
                domain_status: self.domain_status,
                whois_raw_registry: self.whois_raw_registry,
                domain_registrar: self.domain_registrar,
            },
        )
    }
}
