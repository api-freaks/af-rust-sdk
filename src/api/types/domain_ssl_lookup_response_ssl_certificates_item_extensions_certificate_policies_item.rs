pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItem {
    /// Policy identifier
    #[serde(rename = "policyId")]
    #[serde(default)]
    pub policy_id: String,
    /// Policy qualifier details
    #[serde(rename = "policyQualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_qualifier: Option<
        DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifier,
    >,
}

impl DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItem {
    pub fn builder(
    ) -> DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemBuilder {
        <DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemBuilder {
    policy_id: Option<String>,
    policy_qualifier: Option<
        DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifier,
    >,
}

impl DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemBuilder {
    pub fn policy_id(mut self, value: impl Into<String>) -> Self {
        self.policy_id = Some(value.into());
        self
    }

    pub fn policy_qualifier(
        mut self,
        value: DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifier,
    ) -> Self {
        self.policy_qualifier = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`policy_id`](DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemBuilder::policy_id)
    pub fn build(
        self,
    ) -> Result<
        DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItem,
        BuildError,
    > {
        Ok(
            DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItem {
                policy_id: self
                    .policy_id
                    .ok_or_else(|| BuildError::missing_field("policy_id"))?,
                policy_qualifier: self.policy_qualifier,
            },
        )
    }
}
