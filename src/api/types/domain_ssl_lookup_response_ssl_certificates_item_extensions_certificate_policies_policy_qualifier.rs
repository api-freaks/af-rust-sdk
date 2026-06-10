pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oid: Option<String>,
    #[serde(rename = "cpsUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cps_uri: Option<String>,
    #[serde(rename = "userNotice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_notice: Option<DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNotice>,
}

impl DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifier {
    pub fn builder(
    ) -> DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierBuilder
    {
        <DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierBuilder {
    oid: Option<String>,
    cps_uri: Option<String>,
    user_notice: Option<DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNotice>,
}

impl DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierBuilder {
    pub fn oid(mut self, value: impl Into<String>) -> Self {
        self.oid = Some(value.into());
        self
    }

    pub fn cps_uri(mut self, value: impl Into<String>) -> Self {
        self.cps_uri = Some(value.into());
        self
    }

    pub fn user_notice(
        mut self,
        value: DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNotice,
    ) -> Self {
        self.user_notice = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifier`].
    pub fn build(
        self,
    ) -> Result<
        DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifier,
        BuildError,
    > {
        Ok(DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifier {
            oid: self.oid,
            cps_uri: self.cps_uri,
            user_notice: self.user_notice,
        })
    }
}
