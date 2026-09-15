pub use crate::prelude::*;

/// Registrar of record for a domain, as published by either the registrar or the registry.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactDomainRegistrar {
    /// IANA registrar ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iana_id: Option<String>,
    /// Registrar identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Type of registrar ID (e.g. IANA).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_type: Option<String>,
    /// Registrar handle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    /// Registry-specific registrar ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// Registry authority name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authoritative_registry_name: Option<String>,
    /// Registrar organization number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_number: Option<String>,
    /// Indicates if the registrar is a sponsor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sponsor: Option<bool>,
    /// Registrar's ICANN accreditation status (e.g. accredited), when published at the registrar level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Registered name of the registrar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrar_name: Option<String>,
    /// Lowercased, normalized form of the registrar name, when published at the registrar level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_name: Option<String>,
    /// WHOIS server operated by the registrar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whois_server: Option<String>,
    /// RDAP server URL operated by the registrar, when published at the registrar level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdap_server: Option<String>,
    /// Registrar's website URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>,
    /// Registrar abuse or contact email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// Registrar contact phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

impl BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactDomainRegistrar {
    pub fn builder(
    ) -> BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactDomainRegistrarBuilder
    {
        <BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactDomainRegistrarBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactDomainRegistrarBuilder {
    iana_id: Option<String>,
    id: Option<String>,
    id_type: Option<String>,
    handle: Option<String>,
    registry_id: Option<String>,
    authoritative_registry_name: Option<String>,
    organization_number: Option<String>,
    is_sponsor: Option<bool>,
    status: Option<String>,
    registrar_name: Option<String>,
    normalized_name: Option<String>,
    whois_server: Option<String>,
    rdap_server: Option<String>,
    website_url: Option<String>,
    email_address: Option<String>,
    phone_number: Option<String>,
}

impl BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactDomainRegistrarBuilder {
    pub fn iana_id(mut self, value: impl Into<String>) -> Self {
        self.iana_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn id_type(mut self, value: impl Into<String>) -> Self {
        self.id_type = Some(value.into());
        self
    }

    pub fn handle(mut self, value: impl Into<String>) -> Self {
        self.handle = Some(value.into());
        self
    }

    pub fn registry_id(mut self, value: impl Into<String>) -> Self {
        self.registry_id = Some(value.into());
        self
    }

    pub fn authoritative_registry_name(mut self, value: impl Into<String>) -> Self {
        self.authoritative_registry_name = Some(value.into());
        self
    }

    pub fn organization_number(mut self, value: impl Into<String>) -> Self {
        self.organization_number = Some(value.into());
        self
    }

    pub fn is_sponsor(mut self, value: bool) -> Self {
        self.is_sponsor = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn registrar_name(mut self, value: impl Into<String>) -> Self {
        self.registrar_name = Some(value.into());
        self
    }

    pub fn normalized_name(mut self, value: impl Into<String>) -> Self {
        self.normalized_name = Some(value.into());
        self
    }

    pub fn whois_server(mut self, value: impl Into<String>) -> Self {
        self.whois_server = Some(value.into());
        self
    }

    pub fn rdap_server(mut self, value: impl Into<String>) -> Self {
        self.rdap_server = Some(value.into());
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

    /// Consumes the builder and constructs a [`BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactDomainRegistrar`].
    pub fn build(
        self,
    ) -> Result<
        BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactDomainRegistrar,
        BuildError,
    > {
        Ok(
            BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactDomainRegistrar {
                iana_id: self.iana_id,
                id: self.id,
                id_type: self.id_type,
                handle: self.handle,
                registry_id: self.registry_id,
                authoritative_registry_name: self.authoritative_registry_name,
                organization_number: self.organization_number,
                is_sponsor: self.is_sponsor,
                status: self.status,
                registrar_name: self.registrar_name,
                normalized_name: self.normalized_name,
                whois_server: self.whois_server,
                rdap_server: self.rdap_server,
                website_url: self.website_url,
                email_address: self.email_address,
                phone_number: self.phone_number,
            },
        )
    }
}
