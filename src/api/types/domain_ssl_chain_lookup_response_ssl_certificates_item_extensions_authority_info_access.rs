pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainSslChainLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccess {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocsp: Option<Vec<String>>,
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
    pub fn build(
        self,
    ) -> Result<
        DomainSslChainLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccess,
        BuildError,
    > {
        Ok(
            DomainSslChainLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccess {
                issuers: self.issuers,
                ocsp: self.ocsp,
            },
        )
    }
}
