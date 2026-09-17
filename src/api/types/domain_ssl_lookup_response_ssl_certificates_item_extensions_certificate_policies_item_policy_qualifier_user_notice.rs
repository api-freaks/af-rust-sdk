pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNotice {
    /// Explicit text notice
    #[serde(rename = "explicitText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_text: Option<String>,
    #[serde(rename = "noticeRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notice_ref: Option<DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNoticeNoticeRef>,
}

impl DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNotice {
    pub fn builder() -> DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNoticeBuilder {
        <DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNoticeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNoticeBuilder {
    explicit_text: Option<String>,
    notice_ref: Option<DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNoticeNoticeRef>,
}

impl DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNoticeBuilder {
    pub fn explicit_text(mut self, value: impl Into<String>) -> Self {
        self.explicit_text = Some(value.into());
        self
    }

    pub fn notice_ref(mut self, value: DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNoticeNoticeRef) -> Self {
        self.notice_ref = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNotice`].
    pub fn build(self) -> Result<DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNotice, BuildError> {
        Ok(DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifierUserNotice {
            explicit_text: self.explicit_text,
            notice_ref: self.notice_ref,
        })
    }
}
