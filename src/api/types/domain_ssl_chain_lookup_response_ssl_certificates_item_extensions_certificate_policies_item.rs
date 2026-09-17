pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItem {
    /// Policy identifier
    #[serde(rename = "policyId")]
    #[serde(default)]
    pub policy_id: String,
    /// Policy qualifier details
    #[serde(rename = "policyQualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_qualifier: Option<DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifier>,
}

impl DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItem {
    pub fn builder(
    ) -> DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemBuilder
    {
        <DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemBuilder {
    policy_id: Option<String>,
    policy_qualifier: Option<DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifier>,
}

impl DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemBuilder {
    pub fn policy_id(mut self, value: impl Into<String>) -> Self {
        self.policy_id = Some(value.into());
        self
    }

    pub fn policy_qualifier(
        mut self,
        value: DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemPolicyQualifier,
    ) -> Self {
        self.policy_qualifier = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`policy_id`](DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItemBuilder::policy_id)
    pub fn build(
        self,
    ) -> Result<
        DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItem,
        BuildError,
    > {
        Ok(
            DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesItem {
                policy_id: self
                    .policy_id
                    .ok_or_else(|| BuildError::missing_field("policy_id"))?,
                policy_qualifier: self.policy_qualifier,
            },
        )
    }
}
