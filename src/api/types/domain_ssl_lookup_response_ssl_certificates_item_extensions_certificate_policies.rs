pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePolicies {
    #[serde(rename = "policyId")]
    #[serde(default)]
    pub policy_id: String,
    #[serde(rename = "policyQualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_qualifier: Option<
        DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifier,
    >,
}

impl DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePolicies {
    pub fn builder(
    ) -> DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesBuilder {
        <DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesBuilder {
    policy_id: Option<String>,
    policy_qualifier: Option<
        DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifier,
    >,
}

impl DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesBuilder {
    pub fn policy_id(mut self, value: impl Into<String>) -> Self {
        self.policy_id = Some(value.into());
        self
    }

    pub fn policy_qualifier(
        mut self,
        value: DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesPolicyQualifier,
    ) -> Self {
        self.policy_qualifier = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePolicies`].
    /// This method will fail if any of the following fields are not set:
    /// - [`policy_id`](DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePoliciesBuilder::policy_id)
    pub fn build(
        self,
    ) -> Result<DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePolicies, BuildError>
    {
        Ok(
            DomainSslLookupResponseSslCertificatesItemExtensionsCertificatePolicies {
                policy_id: self
                    .policy_id
                    .ok_or_else(|| BuildError::missing_field("policy_id"))?,
                policy_qualifier: self.policy_qualifier,
            },
        )
    }
}
