pub use crate::prelude::*;

/// Registry-level (as opposed to registrar-level) WHOIS data, sourced directly from the TLD registry.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainWhoisLookupV2ResponseRegistryData {
    /// Domain name as recorded by the registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// Timestamp when the registry-level record was queried (format YYYY-MM-DD HH:mm:ss, not ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_time: Option<String>,
    /// Registry WHOIS server that returned this data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whois_server: Option<String>,
    /// Domain registration status as recorded by the registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_registered: Option<DomainWhoisLookupV2ResponseRegistryDataDomainRegistered>,
    /// Domain creation date as recorded by the registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<NaiveDate>,
    /// Domain last-updated date as recorded by the registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<NaiveDate>,
    /// Domain expiry date as recorded by the registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<NaiveDate>,
    /// Name servers as recorded by the registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_servers: Option<Vec<String>>,
    /// EPP domain status codes as recorded by the registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_status: Option<Vec<String>>,
    /// Raw WHOIS text as returned directly by the registry server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whois_raw_registery: Option<String>,
    /// Registrar of record for a domain, as published by either the registrar or the registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_registrar: Option<DomainWhoisLookupV2ResponseRegistryDataDomainRegistrar>,
}

impl DomainWhoisLookupV2ResponseRegistryData {
    pub fn builder() -> DomainWhoisLookupV2ResponseRegistryDataBuilder {
        <DomainWhoisLookupV2ResponseRegistryDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainWhoisLookupV2ResponseRegistryDataBuilder {
    domain_name: Option<String>,
    query_time: Option<String>,
    whois_server: Option<String>,
    domain_registered: Option<DomainWhoisLookupV2ResponseRegistryDataDomainRegistered>,
    create_date: Option<NaiveDate>,
    update_date: Option<NaiveDate>,
    expiry_date: Option<NaiveDate>,
    name_servers: Option<Vec<String>>,
    domain_status: Option<Vec<String>>,
    whois_raw_registery: Option<String>,
    domain_registrar: Option<DomainWhoisLookupV2ResponseRegistryDataDomainRegistrar>,
}

impl DomainWhoisLookupV2ResponseRegistryDataBuilder {
    pub fn domain_name(mut self, value: impl Into<String>) -> Self {
        self.domain_name = Some(value.into());
        self
    }

    pub fn query_time(mut self, value: impl Into<String>) -> Self {
        self.query_time = Some(value.into());
        self
    }

    pub fn whois_server(mut self, value: impl Into<String>) -> Self {
        self.whois_server = Some(value.into());
        self
    }

    pub fn domain_registered(
        mut self,
        value: DomainWhoisLookupV2ResponseRegistryDataDomainRegistered,
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

    pub fn whois_raw_registery(mut self, value: impl Into<String>) -> Self {
        self.whois_raw_registery = Some(value.into());
        self
    }

    pub fn domain_registrar(
        mut self,
        value: DomainWhoisLookupV2ResponseRegistryDataDomainRegistrar,
    ) -> Self {
        self.domain_registrar = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainWhoisLookupV2ResponseRegistryData`].
    pub fn build(self) -> Result<DomainWhoisLookupV2ResponseRegistryData, BuildError> {
        Ok(DomainWhoisLookupV2ResponseRegistryData {
            domain_name: self.domain_name,
            query_time: self.query_time,
            whois_server: self.whois_server,
            domain_registered: self.domain_registered,
            create_date: self.create_date,
            update_date: self.update_date,
            expiry_date: self.expiry_date,
            name_servers: self.name_servers,
            domain_status: self.domain_status,
            whois_raw_registery: self.whois_raw_registery,
            domain_registrar: self.domain_registrar,
        })
    }
}
