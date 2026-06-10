pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainSslChainLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccess {
    #[serde(default)]
    pub issuers: Vec<String>,
    #[serde(default)]
    pub ocsp: Vec<String>,
}

impl DomainSslChainLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccess {
    pub fn builder(
    ) -> DomainSslChainLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccessBuilder {
        <DomainSslChainLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccessBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainSslChainLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccessBuilder {
    issuers: Option<Vec<String>>,
    ocsp: Option<Vec<String>>,
}

impl DomainSslChainLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccessBuilder {
    pub fn issuers(mut self, value: Vec<String>) -> Self {
        self.issuers = Some(value);
        self
    }

    pub fn ocsp(mut self, value: Vec<String>) -> Self {
        self.ocsp = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainSslChainLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccess`].
    /// This method will fail if any of the following fields are not set:
    /// - [`issuers`](DomainSslChainLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccessBuilder::issuers)
    /// - [`ocsp`](DomainSslChainLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccessBuilder::ocsp)
    pub fn build(
        self,
    ) -> Result<
        DomainSslChainLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccess,
        BuildError,
    > {
        Ok(
            DomainSslChainLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccess {
                issuers: self
                    .issuers
                    .ok_or_else(|| BuildError::missing_field("issuers"))?,
                ocsp: self.ocsp.ok_or_else(|| BuildError::missing_field("ocsp"))?,
            },
        )
    }
}
