pub use crate::prelude::*;

/// Policy qualifier details
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifier {
    /// Object identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oid: Option<String>,
    /// URI of the CPS
    #[serde(rename = "cpsUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cps_uri: Option<String>,
    #[serde(rename = "userNotice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_notice: Option<DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNotice>,
}

impl
    DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifier
{
    pub fn builder() -> DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierBuilder{
        <DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierBuilder {
    oid: Option<String>,
    cps_uri: Option<String>,
    user_notice: Option<DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNotice>,
}

impl DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierBuilder {
    pub fn oid(mut self, value: impl Into<String>) -> Self {
        self.oid = Some(value.into());
        self
    }

    pub fn cps_uri(mut self, value: impl Into<String>) -> Self {
        self.cps_uri = Some(value.into());
        self
    }

    pub fn user_notice(mut self, value: DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNotice) -> Self {
        self.user_notice = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifier`].
    pub fn build(self) -> Result<DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifier, BuildError> {
        Ok(DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifier {
            oid: self.oid,
            cps_uri: self.cps_uri,
            user_notice: self.user_notice,
        })
    }
}
