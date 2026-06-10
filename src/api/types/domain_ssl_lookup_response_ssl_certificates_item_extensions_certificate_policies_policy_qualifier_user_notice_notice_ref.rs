pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNoticeNoticeRef
{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(rename = "noticeNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notice_numbers: Option<String>,
}

impl DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNoticeNoticeRef {
    pub fn builder() -> DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNoticeNoticeRefBuilder {
        <DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNoticeNoticeRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNoticeNoticeRefBuilder
{
    organization: Option<String>,
    notice_numbers: Option<String>,
}

impl DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNoticeNoticeRefBuilder {
    pub fn organization(mut self, value: impl Into<String>) -> Self {
        self.organization = Some(value.into());
        self
    }

    pub fn notice_numbers(mut self, value: impl Into<String>) -> Self {
        self.notice_numbers = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNoticeNoticeRef`].
    pub fn build(self) -> Result<DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNoticeNoticeRef, BuildError> {
        Ok(DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNoticeNoticeRef {
            organization: self.organization,
            notice_numbers: self.notice_numbers,
        })
    }
}
