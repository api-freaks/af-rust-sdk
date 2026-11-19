pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactRegistryDataDomainRegistrar
{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iana_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrar_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whois_server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// Normalized (lowercase, stripped) registrar name, when published.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_name: Option<String>,
    /// RDAP (Registration Data Access Protocol) server URL of the registrar, when published.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdap_server: Option<String>,
    /// Accreditation status of the registrar (e.g., 'accredited'), when published.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactRegistryDataDomainRegistrar {
    pub fn builder() -> DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactRegistryDataDomainRegistrarBuilder {
        <DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactRegistryDataDomainRegistrarBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactRegistryDataDomainRegistrarBuilder
{
    iana_id: Option<String>,
    registrar_name: Option<String>,
    whois_server: Option<String>,
    website_url: Option<String>,
    email_address: Option<String>,
    phone_number: Option<String>,
    normalized_name: Option<String>,
    rdap_server: Option<String>,
    status: Option<String>,
}

impl DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactRegistryDataDomainRegistrarBuilder {
    pub fn iana_id(mut self, value: impl Into<String>) -> Self {
        self.iana_id = Some(value.into());
        self
    }

    pub fn registrar_name(mut self, value: impl Into<String>) -> Self {
        self.registrar_name = Some(value.into());
        self
    }

    pub fn whois_server(mut self, value: impl Into<String>) -> Self {
        self.whois_server = Some(value.into());
        self
    }

    pub fn website_url(mut self, value: impl Into<String>) -> Self {
        self.website_url = Some(value.into());
        self
    }

    pub fn email_address(mut self, value: impl Into<String>) -> Self {
        self.email_address = Some(value.into());
        self
    }

    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(value.into());
        self
    }

    pub fn normalized_name(mut self, value: impl Into<String>) -> Self {
        self.normalized_name = Some(value.into());
        self
    }

    pub fn rdap_server(mut self, value: impl Into<String>) -> Self {
        self.rdap_server = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactRegistryDataDomainRegistrar`].
    pub fn build(self) -> Result<DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactRegistryDataDomainRegistrar, BuildError> {
        Ok(DomainWhoisReverseResponseWhoisDomainsHistoricalItemAdministrativeContactRegistryDataDomainRegistrar {
            iana_id: self.iana_id,
            registrar_name: self.registrar_name,
            whois_server: self.whois_server,
            website_url: self.website_url,
            email_address: self.email_address,
            phone_number: self.phone_number,
            normalized_name: self.normalized_name,
            rdap_server: self.rdap_server,
            status: self.status,
        })
    }
}
