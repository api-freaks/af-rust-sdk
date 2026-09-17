pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNoticeNoticeRef
{
    /// Organization providing the notice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// Notice numbers
    #[serde(rename = "noticeNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notice_numbers: Option<String>,
}

impl DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNoticeNoticeRef {
    pub fn builder() -> DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNoticeNoticeRefBuilder {
        <DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNoticeNoticeRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNoticeNoticeRefBuilder
{
    organization: Option<String>,
    notice_numbers: Option<String>,
}

impl DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNoticeNoticeRefBuilder {
    pub fn organization(mut self, value: impl Into<String>) -> Self {
        self.organization = Some(value.into());
        self
    }

    pub fn notice_numbers(mut self, value: impl Into<String>) -> Self {
        self.notice_numbers = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNoticeNoticeRef`].
    pub fn build(self) -> Result<DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNoticeNoticeRef, BuildError> {
        Ok(DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNoticeNoticeRef {
            organization: self.organization,
            notice_numbers: self.notice_numbers,
        })
    }
}
