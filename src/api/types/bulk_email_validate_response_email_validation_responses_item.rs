pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BulkEmailValidateResponseEmailValidationResponsesItem {
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "validEmail")]
    pub valid_email: BulkEmailValidateResponseEmailValidationResponsesItemValidEmail,
    #[serde(rename = "validSyntax")]
    #[serde(default)]
    pub valid_syntax: bool,
    #[serde(default)]
    pub domain: BulkEmailValidateResponseEmailValidationResponsesItemDomain,
    #[serde(default)]
    pub account: BulkEmailValidateResponseEmailValidationResponsesItemAccount,
    #[serde(default)]
    pub dns: BulkEmailValidateResponseEmailValidationResponsesItemDns,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<BulkEmailValidateResponseEmailValidationResponsesItemAddress>,
}

impl BulkEmailValidateResponseEmailValidationResponsesItem {
    pub fn builder() -> BulkEmailValidateResponseEmailValidationResponsesItemBuilder {
        <BulkEmailValidateResponseEmailValidationResponsesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkEmailValidateResponseEmailValidationResponsesItemBuilder {
    success: Option<bool>,
    email: Option<String>,
    name: Option<String>,
    reason: Option<String>,
    valid_email: Option<BulkEmailValidateResponseEmailValidationResponsesItemValidEmail>,
    valid_syntax: Option<bool>,
    domain: Option<BulkEmailValidateResponseEmailValidationResponsesItemDomain>,
    account: Option<BulkEmailValidateResponseEmailValidationResponsesItemAccount>,
    dns: Option<BulkEmailValidateResponseEmailValidationResponsesItemDns>,
    ip: Option<String>,
    address: Option<BulkEmailValidateResponseEmailValidationResponsesItemAddress>,
}

impl BulkEmailValidateResponseEmailValidationResponsesItemBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn valid_email(
        mut self,
        value: BulkEmailValidateResponseEmailValidationResponsesItemValidEmail,
    ) -> Self {
        self.valid_email = Some(value);
        self
    }

    pub fn valid_syntax(mut self, value: bool) -> Self {
        self.valid_syntax = Some(value);
        self
    }

    pub fn domain(
        mut self,
        value: BulkEmailValidateResponseEmailValidationResponsesItemDomain,
    ) -> Self {
        self.domain = Some(value);
        self
    }

    pub fn account(
        mut self,
        value: BulkEmailValidateResponseEmailValidationResponsesItemAccount,
    ) -> Self {
        self.account = Some(value);
        self
    }

    pub fn dns(mut self, value: BulkEmailValidateResponseEmailValidationResponsesItemDns) -> Self {
        self.dns = Some(value);
        self
    }

    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn address(
        mut self,
        value: BulkEmailValidateResponseEmailValidationResponsesItemAddress,
    ) -> Self {
        self.address = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkEmailValidateResponseEmailValidationResponsesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](BulkEmailValidateResponseEmailValidationResponsesItemBuilder::success)
    /// - [`email`](BulkEmailValidateResponseEmailValidationResponsesItemBuilder::email)
    /// - [`valid_email`](BulkEmailValidateResponseEmailValidationResponsesItemBuilder::valid_email)
    /// - [`valid_syntax`](BulkEmailValidateResponseEmailValidationResponsesItemBuilder::valid_syntax)
    /// - [`domain`](BulkEmailValidateResponseEmailValidationResponsesItemBuilder::domain)
    /// - [`account`](BulkEmailValidateResponseEmailValidationResponsesItemBuilder::account)
    /// - [`dns`](BulkEmailValidateResponseEmailValidationResponsesItemBuilder::dns)
    pub fn build(
        self,
    ) -> Result<BulkEmailValidateResponseEmailValidationResponsesItem, BuildError> {
        Ok(BulkEmailValidateResponseEmailValidationResponsesItem {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            name: self.name,
            reason: self.reason,
            valid_email: self
                .valid_email
                .ok_or_else(|| BuildError::missing_field("valid_email"))?,
            valid_syntax: self
                .valid_syntax
                .ok_or_else(|| BuildError::missing_field("valid_syntax"))?,
            domain: self
                .domain
                .ok_or_else(|| BuildError::missing_field("domain"))?,
            account: self
                .account
                .ok_or_else(|| BuildError::missing_field("account"))?,
            dns: self.dns.ok_or_else(|| BuildError::missing_field("dns"))?,
            ip: self.ip,
            address: self.address,
        })
    }
}
