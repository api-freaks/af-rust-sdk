pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNotice {
    #[serde(rename = "explicitText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_text: Option<String>,
    #[serde(rename = "noticeRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notice_ref: Option<DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNoticeNoticeRef>,
}

impl DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNotice {
    pub fn builder() -> DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNoticeBuilder {
        <DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNoticeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNoticeBuilder {
    explicit_text: Option<String>,
    notice_ref: Option<DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNoticeNoticeRef>,
}

impl DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNoticeBuilder {
    pub fn explicit_text(mut self, value: impl Into<String>) -> Self {
        self.explicit_text = Some(value.into());
        self
    }

    pub fn notice_ref(mut self, value: DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNoticeNoticeRef) -> Self {
        self.notice_ref = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNotice`].
    pub fn build(self) -> Result<DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNotice, BuildError> {
        Ok(DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifierUserNotice {
            explicit_text: self.explicit_text,
            notice_ref: self.notice_ref,
        })
    }
}
