pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNotice {
    #[serde(rename = "explicitText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_text: Option<String>,
    #[serde(rename = "noticeRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notice_ref: Option<DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNoticeNoticeRef>,
}

impl
    DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNotice
{
    pub fn builder() -> DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNoticeBuilder{
        <DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNoticeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNoticeBuilder {
    explicit_text: Option<String>,
    notice_ref: Option<DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNoticeNoticeRef>,
}

impl DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNoticeBuilder {
    pub fn explicit_text(mut self, value: impl Into<String>) -> Self {
        self.explicit_text = Some(value.into());
        self
    }

    pub fn notice_ref(mut self, value: DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNoticeNoticeRef) -> Self {
        self.notice_ref = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNotice`].
    pub fn build(self) -> Result<DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNotice, BuildError> {
        Ok(DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNotice {
            explicit_text: self.explicit_text,
            notice_ref: self.notice_ref,
        })
    }
}
