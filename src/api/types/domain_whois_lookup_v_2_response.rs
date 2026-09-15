pub use crate::prelude::*;

/// Current WHOIS registration record for the requested domain.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DomainWhoisLookupV2Response {
    /// true if the request was successfully processed.
    #[serde(default)]
    pub status: bool,
    /// Domain name that was queried.
    #[serde(default)]
    pub domain_name: String,
    /// Timestamp when the WHOIS query was executed.
    #[serde(default)]
    pub query_time: String,
    /// WHOIS or RDAP server that provided this record.
    #[serde(default)]
    pub whois_server: String,
    /// Domain registration status; 'restricted' means the registry withholds registration details.
    pub domain_registered: DomainWhoisLookupV2ResponseDomainRegistered,
    /// Indicates if DNSSEC or secure DNS is enabled for the domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_dns: Option<bool>,
    /// Internal domain registry handle/ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_handle: Option<String>,
    /// Date the domain was originally registered, when the domain is registered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<NaiveDate>,
    /// Date the domain registration was last updated, when the domain is registered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<NaiveDate>,
    /// Date the domain registration is set to expire, when the domain is registered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<NaiveDate>,
    /// Registrar of record for a domain, as published by either the registrar or the registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_registrar: Option<DomainWhoisLookupV2ResponseDomainRegistrar>,
    /// A contact record (registrant, administrative, technical, billing, or reseller) published in the domain's WHOIS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reseller_contact: Option<DomainWhoisLookupV2ResponseResellerContact>,
    /// A contact record (registrant, administrative, technical, billing, or reseller) published in the domain's WHOIS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_contact: Option<DomainWhoisLookupV2ResponseRegistrantContact>,
    /// A contact record (registrant, administrative, technical, billing, or reseller) published in the domain's WHOIS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_contact: Option<DomainWhoisLookupV2ResponseAdministrativeContact>,
    /// A contact record (registrant, administrative, technical, billing, or reseller) published in the domain's WHOIS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_contact: Option<DomainWhoisLookupV2ResponseTechnicalContact>,
    /// A contact record (registrant, administrative, technical, billing, or reseller) published in the domain's WHOIS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_contact: Option<DomainWhoisLookupV2ResponseBillingContact>,
    /// Registrar's abuse-reporting contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abuse_contact: Option<DomainWhoisLookupV2ResponseAbuseContact>,
    /// Domain eligibility information (populated for TLDs with registrant eligibility requirements, e.g. .eu).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eligibility_info: Option<DomainWhoisLookupV2ResponseEligibilityInfo>,
    /// Name servers currently recorded for the domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_servers: Option<Vec<String>>,
    /// EPP domain status codes currently recorded for the domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_status: Option<Vec<String>>,
    /// Raw WHOIS text as returned by the registrar's WHOIS server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whois_raw_domain: Option<String>,
    /// Registry-level (as opposed to registrar-level) WHOIS data, sourced directly from the TLD registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_data: Option<DomainWhoisLookupV2ResponseRegistryData>,
}

impl DomainWhoisLookupV2Response {
    pub fn builder() -> DomainWhoisLookupV2ResponseBuilder {
        <DomainWhoisLookupV2ResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainWhoisLookupV2ResponseBuilder {
    status: Option<bool>,
    domain_name: Option<String>,
    query_time: Option<String>,
    whois_server: Option<String>,
    domain_registered: Option<DomainWhoisLookupV2ResponseDomainRegistered>,
    secure_dns: Option<bool>,
    domain_handle: Option<String>,
    create_date: Option<NaiveDate>,
    update_date: Option<NaiveDate>,
    expiry_date: Option<NaiveDate>,
    domain_registrar: Option<DomainWhoisLookupV2ResponseDomainRegistrar>,
    reseller_contact: Option<DomainWhoisLookupV2ResponseResellerContact>,
    registrant_contact: Option<DomainWhoisLookupV2ResponseRegistrantContact>,
    administrative_contact: Option<DomainWhoisLookupV2ResponseAdministrativeContact>,
    technical_contact: Option<DomainWhoisLookupV2ResponseTechnicalContact>,
    billing_contact: Option<DomainWhoisLookupV2ResponseBillingContact>,
    abuse_contact: Option<DomainWhoisLookupV2ResponseAbuseContact>,
    eligibility_info: Option<DomainWhoisLookupV2ResponseEligibilityInfo>,
    name_servers: Option<Vec<String>>,
    domain_status: Option<Vec<String>>,
    whois_raw_domain: Option<String>,
    registry_data: Option<DomainWhoisLookupV2ResponseRegistryData>,
}

impl DomainWhoisLookupV2ResponseBuilder {
    pub fn status(mut self, value: bool) -> Self {
        self.status = Some(value);
        self
    }

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

    pub fn domain_registered(mut self, value: DomainWhoisLookupV2ResponseDomainRegistered) -> Self {
        self.domain_registered = Some(value);
        self
    }

    pub fn secure_dns(mut self, value: bool) -> Self {
        self.secure_dns = Some(value);
        self
    }

    pub fn domain_handle(mut self, value: impl Into<String>) -> Self {
        self.domain_handle = Some(value.into());
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

    pub fn domain_registrar(mut self, value: DomainWhoisLookupV2ResponseDomainRegistrar) -> Self {
        self.domain_registrar = Some(value);
        self
    }

    pub fn reseller_contact(mut self, value: DomainWhoisLookupV2ResponseResellerContact) -> Self {
        self.reseller_contact = Some(value);
        self
    }

    pub fn registrant_contact(
        mut self,
        value: DomainWhoisLookupV2ResponseRegistrantContact,
    ) -> Self {
        self.registrant_contact = Some(value);
        self
    }

    pub fn administrative_contact(
        mut self,
        value: DomainWhoisLookupV2ResponseAdministrativeContact,
    ) -> Self {
        self.administrative_contact = Some(value);
        self
    }

    pub fn technical_contact(mut self, value: DomainWhoisLookupV2ResponseTechnicalContact) -> Self {
        self.technical_contact = Some(value);
        self
    }

    pub fn billing_contact(mut self, value: DomainWhoisLookupV2ResponseBillingContact) -> Self {
        self.billing_contact = Some(value);
        self
    }

    pub fn abuse_contact(mut self, value: DomainWhoisLookupV2ResponseAbuseContact) -> Self {
        self.abuse_contact = Some(value);
        self
    }

    pub fn eligibility_info(mut self, value: DomainWhoisLookupV2ResponseEligibilityInfo) -> Self {
        self.eligibility_info = Some(value);
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

    pub fn registry_data(mut self, value: DomainWhoisLookupV2ResponseRegistryData) -> Self {
        self.registry_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainWhoisLookupV2Response`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](DomainWhoisLookupV2ResponseBuilder::status)
    /// - [`domain_name`](DomainWhoisLookupV2ResponseBuilder::domain_name)
    /// - [`query_time`](DomainWhoisLookupV2ResponseBuilder::query_time)
    /// - [`whois_server`](DomainWhoisLookupV2ResponseBuilder::whois_server)
    /// - [`domain_registered`](DomainWhoisLookupV2ResponseBuilder::domain_registered)
    pub fn build(self) -> Result<DomainWhoisLookupV2Response, BuildError> {
        Ok(DomainWhoisLookupV2Response {
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
            secure_dns: self.secure_dns,
            domain_handle: self.domain_handle,
            create_date: self.create_date,
            update_date: self.update_date,
            expiry_date: self.expiry_date,
            domain_registrar: self.domain_registrar,
            reseller_contact: self.reseller_contact,
            registrant_contact: self.registrant_contact,
            administrative_contact: self.administrative_contact,
            technical_contact: self.technical_contact,
            billing_contact: self.billing_contact,
            abuse_contact: self.abuse_contact,
            eligibility_info: self.eligibility_info,
            name_servers: self.name_servers,
            domain_status: self.domain_status,
            whois_raw_domain: self.whois_raw_domain,
            registry_data: self.registry_data,
        })
    }
}
