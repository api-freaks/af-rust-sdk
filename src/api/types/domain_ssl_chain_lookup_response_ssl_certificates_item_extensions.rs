pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainSslChainLookupResponseSslCertificatesItemExtensions {
    #[serde(rename = "authorityKeyIdentifier")]
    #[serde(default)]
    pub authority_key_identifier: String,
    #[serde(rename = "subjectKeyIdentifier")]
    #[serde(default)]
    pub subject_key_identifier: String,
    #[serde(rename = "keyUsages")]
    #[serde(default)]
    pub key_usages: Vec<String>,
    #[serde(rename = "extendedKeyUsages")]
    #[serde(default)]
    pub extended_key_usages: Vec<String>,
    #[serde(rename = "crlDistributionPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crl_distribution_points: Option<Vec<String>>,
    #[serde(rename = "authorityInfoAccess")]
    #[serde(default)]
    pub authority_info_access:
        DomainSslChainLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccess,
    #[serde(rename = "subjectAlternativeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_names:
        Option<DomainSslChainLookupResponseSslCertificatesItemExtensionsSubjectAlternativeNames>,
    #[serde(rename = "certificatePolicies")]
    #[serde(default)]
    pub certificate_policies:
        DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePolicies,
}

impl DomainSslChainLookupResponseSslCertificatesItemExtensions {
    pub fn builder() -> DomainSslChainLookupResponseSslCertificatesItemExtensionsBuilder {
        <DomainSslChainLookupResponseSslCertificatesItemExtensionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainSslChainLookupResponseSslCertificatesItemExtensionsBuilder {
    authority_key_identifier: Option<String>,
    subject_key_identifier: Option<String>,
    key_usages: Option<Vec<String>>,
    extended_key_usages: Option<Vec<String>>,
    crl_distribution_points: Option<Vec<String>>,
    authority_info_access:
        Option<DomainSslChainLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccess>,
    subject_alternative_names:
        Option<DomainSslChainLookupResponseSslCertificatesItemExtensionsSubjectAlternativeNames>,
    certificate_policies:
        Option<DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePolicies>,
}

impl DomainSslChainLookupResponseSslCertificatesItemExtensionsBuilder {
    pub fn authority_key_identifier(mut self, value: impl Into<String>) -> Self {
        self.authority_key_identifier = Some(value.into());
        self
    }

    pub fn subject_key_identifier(mut self, value: impl Into<String>) -> Self {
        self.subject_key_identifier = Some(value.into());
        self
    }

    pub fn key_usages(mut self, value: Vec<String>) -> Self {
        self.key_usages = Some(value);
        self
    }

    pub fn extended_key_usages(mut self, value: Vec<String>) -> Self {
        self.extended_key_usages = Some(value);
        self
    }

    pub fn crl_distribution_points(mut self, value: Vec<String>) -> Self {
        self.crl_distribution_points = Some(value);
        self
    }

    pub fn authority_info_access(
        mut self,
        value: DomainSslChainLookupResponseSslCertificatesItemExtensionsAuthorityInfoAccess,
    ) -> Self {
        self.authority_info_access = Some(value);
        self
    }

    pub fn subject_alternative_names(
        mut self,
        value: DomainSslChainLookupResponseSslCertificatesItemExtensionsSubjectAlternativeNames,
    ) -> Self {
        self.subject_alternative_names = Some(value);
        self
    }

    pub fn certificate_policies(
        mut self,
        value: DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePolicies,
    ) -> Self {
        self.certificate_policies = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainSslChainLookupResponseSslCertificatesItemExtensions`].
    /// This method will fail if any of the following fields are not set:
    /// - [`authority_key_identifier`](DomainSslChainLookupResponseSslCertificatesItemExtensionsBuilder::authority_key_identifier)
    /// - [`subject_key_identifier`](DomainSslChainLookupResponseSslCertificatesItemExtensionsBuilder::subject_key_identifier)
    /// - [`key_usages`](DomainSslChainLookupResponseSslCertificatesItemExtensionsBuilder::key_usages)
    /// - [`extended_key_usages`](DomainSslChainLookupResponseSslCertificatesItemExtensionsBuilder::extended_key_usages)
    /// - [`authority_info_access`](DomainSslChainLookupResponseSslCertificatesItemExtensionsBuilder::authority_info_access)
    /// - [`certificate_policies`](DomainSslChainLookupResponseSslCertificatesItemExtensionsBuilder::certificate_policies)
    pub fn build(
        self,
    ) -> Result<DomainSslChainLookupResponseSslCertificatesItemExtensions, BuildError> {
        Ok(DomainSslChainLookupResponseSslCertificatesItemExtensions {
            authority_key_identifier: self
                .authority_key_identifier
                .ok_or_else(|| BuildError::missing_field("authority_key_identifier"))?,
            subject_key_identifier: self
                .subject_key_identifier
                .ok_or_else(|| BuildError::missing_field("subject_key_identifier"))?,
            key_usages: self
                .key_usages
                .ok_or_else(|| BuildError::missing_field("key_usages"))?,
            extended_key_usages: self
                .extended_key_usages
                .ok_or_else(|| BuildError::missing_field("extended_key_usages"))?,
            crl_distribution_points: self.crl_distribution_points,
            authority_info_access: self
                .authority_info_access
                .ok_or_else(|| BuildError::missing_field("authority_info_access"))?,
            subject_alternative_names: self.subject_alternative_names,
            certificate_policies: self
                .certificate_policies
                .ok_or_else(|| BuildError::missing_field("certificate_policies"))?,
        })
    }
}
