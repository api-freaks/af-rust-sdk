pub use crate::prelude::*;

/// Registrar's abuse-reporting contact.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactAbuseContact {
    /// Name of the abuse contact at the registrar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrar_name: Option<String>,
    /// Abuse contact email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// Abuse contact phone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

impl BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactAbuseContact {
    pub fn builder(
    ) -> BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactAbuseContactBuilder {
        <BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactAbuseContactBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactAbuseContactBuilder {
    registrar_name: Option<String>,
    email_address: Option<String>,
    phone_number: Option<String>,
}

impl BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactAbuseContactBuilder {
    pub fn registrar_name(mut self, value: impl Into<String>) -> Self {
        self.registrar_name = Some(value.into());
        self
    }

    pub fn email_address(mut self, value: impl Into<String>) -> Self {
        self.email_address = Some(value.into());
        self
    }

    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactAbuseContact`].
    pub fn build(
        self,
    ) -> Result<
        BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactAbuseContact,
        BuildError,
    > {
        Ok(
            BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactAbuseContact {
                registrar_name: self.registrar_name,
                email_address: self.email_address,
                phone_number: self.phone_number,
            },
        )
    }
}
