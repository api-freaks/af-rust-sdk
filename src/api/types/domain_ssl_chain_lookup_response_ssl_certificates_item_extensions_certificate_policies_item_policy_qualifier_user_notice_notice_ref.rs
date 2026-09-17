pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNoticeNoticeRef
{
    /// Organization providing the notice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// Notice numbers
    #[serde(rename = "noticeNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notice_numbers: Option<String>,
}

impl DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNoticeNoticeRef {
    pub fn builder() -> DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNoticeNoticeRefBuilder {
        <DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNoticeNoticeRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNoticeNoticeRefBuilder
{
    organization: Option<String>,
    notice_numbers: Option<String>,
}

impl DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNoticeNoticeRefBuilder {
    pub fn organization(mut self, value: impl Into<String>) -> Self {
        self.organization = Some(value.into());
        self
    }

    pub fn notice_numbers(mut self, value: impl Into<String>) -> Self {
        self.notice_numbers = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNoticeNoticeRef`].
    pub fn build(self) -> Result<DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNoticeNoticeRef, BuildError> {
        Ok(DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNoticeNoticeRef {
            organization: self.organization,
            notice_numbers: self.notice_numbers,
        })
    }
}
