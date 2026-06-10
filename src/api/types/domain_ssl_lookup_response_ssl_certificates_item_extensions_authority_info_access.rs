pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainSslLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccess {
    #[serde(default)]
    pub issuers: Vec<String>,
    #[serde(default)]
    pub ocsp: Vec<String>,
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
    /// This method will fail if any of the following fields are not set:
    /// - [`issuers`](DomainSslLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccessBuilder::issuers)
    /// - [`ocsp`](DomainSslLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccessBuilder::ocsp)
    pub fn build(
        self,
    ) -> Result<DomainSslLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccess, BuildError>
    {
        Ok(
            DomainSslLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccess {
                issuers: self
                    .issuers
                    .ok_or_else(|| BuildError::missing_field("issuers"))?,
                ocsp: self.ocsp.ok_or_else(|| BuildError::missing_field("ocsp"))?,
            },
        )
    }
}
