pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainSslLookupResponseSslCertificatesItemExtensionsSubjectAlternativeNames {
    #[serde(rename = "dnsNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_names: Option<Vec<String>>,
    #[serde(rename = "emailAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_addresses: Option<Vec<String>>,
    #[serde(rename = "ipAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uris: Option<Vec<String>>,
}

impl DomainSslLookupResponseSslCertificatesItemExtensionsSubjectAlternativeNames {
    pub fn builder(
    ) -> DomainSslLookupResponseSslCertificatesItemExtensionsSubjectAlternativeNamesBuilder {
        <DomainSslLookupResponseSslCertificatesItemExtensionsSubjectAlternativeNamesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainSslLookupResponseSslCertificatesItemExtensionsSubjectAlternativeNamesBuilder {
    dns_names: Option<Vec<String>>,
    email_addresses: Option<Vec<String>>,
    ip_addresses: Option<Vec<String>>,
    uris: Option<Vec<String>>,
}

impl DomainSslLookupResponseSslCertificatesItemExtensionsSubjectAlternativeNamesBuilder {
    pub fn dns_names(mut self, value: Vec<String>) -> Self {
        self.dns_names = Some(value);
        self
    }

    pub fn email_addresses(mut self, value: Vec<String>) -> Self {
        self.email_addresses = Some(value);
        self
    }

    pub fn ip_addresses(mut self, value: Vec<String>) -> Self {
        self.ip_addresses = Some(value);
        self
    }

    pub fn uris(mut self, value: Vec<String>) -> Self {
        self.uris = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainSslLookupResponseSslCertificatesItemExtensionsSubjectAlternativeNames`].
    pub fn build(
        self,
    ) -> Result<
        DomainSslLookupResponseSslCertificatesItemExtensionsSubjectAlternativeNames,
        BuildError,
    > {
        Ok(
            DomainSslLookupResponseSslCertificatesItemExtensionsSubjectAlternativeNames {
                dns_names: self.dns_names,
                email_addresses: self.email_addresses,
                ip_addresses: self.ip_addresses,
                uris: self.uris,
            },
        )
    }
}
