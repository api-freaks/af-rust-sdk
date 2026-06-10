pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePolicies {
    #[serde(rename = "policyId")]
    #[serde(default)]
    pub policy_id: String,
    #[serde(rename = "policyQualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_qualifier: Option<
        DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifier,
    >,
}

impl DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePolicies {
    pub fn builder(
    ) -> DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesBuilder {
        <DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesBuilder {
    policy_id: Option<String>,
    policy_qualifier: Option<
        DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifier,
    >,
}

impl DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesBuilder {
    pub fn policy_id(mut self, value: impl Into<String>) -> Self {
        self.policy_id = Some(value.into());
        self
    }

    pub fn policy_qualifier(
        mut self,
        value: DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifier,
    ) -> Self {
        self.policy_qualifier = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePolicies`].
    /// This method will fail if any of the following fields are not set:
    /// - [`policy_id`](DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePoliciesBuilder::policy_id)
    pub fn build(
        self,
    ) -> Result<
        DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePolicies,
        BuildError,
    > {
        Ok(
            DomainSslChainLookupResponseSslCertificatesItemExtensionsCertificatePolicies {
                policy_id: self
                    .policy_id
                    .ok_or_else(|| BuildError::missing_field("policy_id"))?,
                policy_qualifier: self.policy_qualifier,
            },
        )
    }
}
