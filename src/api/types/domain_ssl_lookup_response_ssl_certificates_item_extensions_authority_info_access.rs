pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainSslLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccess {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocsp: Option<Vec<String>>,
}

impl DomainSslLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccess {
    pub fn builder(
    ) -> DomainSslLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccessBuilder {
        <DomainSslLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccessBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainSslLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccessBuilder {
    issuers: Option<Vec<String>>,
    ocsp: Option<Vec<String>>,
}

impl DomainSslLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccessBuilder {
    pub fn issuers(mut self, value: Vec<String>) -> Self {
        self.issuers = Some(value);
        self
    }

    pub fn ocsp(mut self, value: Vec<String>) -> Self {
        self.ocsp = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainSslLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccess`].
    pub fn build(
        self,
    ) -> Result<DomainSslLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccess, BuildError>
    {
        Ok(
            DomainSslLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccess {
                issuers: self.issuers,
                ocsp: self.ocsp,
            },
        )
    }
}
