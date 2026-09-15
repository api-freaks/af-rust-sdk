pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BulkEmailValidateResponseEmailResponseItem {
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "validEmail")]
    pub valid_email: BulkEmailValidateResponseEmailResponseItemValidEmail,
    #[serde(rename = "validSyntax")]
    #[serde(default)]
    pub valid_syntax: bool,
    #[serde(default)]
    pub domain: BulkEmailValidateResponseEmailResponseItemDomain,
    #[serde(default)]
    pub account: BulkEmailValidateResponseEmailResponseItemAccount,
    #[serde(default)]
    pub dns: BulkEmailValidateResponseEmailResponseItemDns,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<BulkEmailValidateResponseEmailResponseItemAddress>,
}

impl BulkEmailValidateResponseEmailResponseItem {
    pub fn builder() -> BulkEmailValidateResponseEmailResponseItemBuilder {
        <BulkEmailValidateResponseEmailResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkEmailValidateResponseEmailResponseItemBuilder {
    success: Option<bool>,
    email: Option<String>,
    name: Option<String>,
    reason: Option<String>,
    valid_email: Option<BulkEmailValidateResponseEmailResponseItemValidEmail>,
    valid_syntax: Option<bool>,
    domain: Option<BulkEmailValidateResponseEmailResponseItemDomain>,
    account: Option<BulkEmailValidateResponseEmailResponseItemAccount>,
    dns: Option<BulkEmailValidateResponseEmailResponseItemDns>,
    ip: Option<String>,
    address: Option<BulkEmailValidateResponseEmailResponseItemAddress>,
}

impl BulkEmailValidateResponseEmailResponseItemBuilder {
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
        value: BulkEmailValidateResponseEmailResponseItemValidEmail,
    ) -> Self {
        self.valid_email = Some(value);
        self
    }

    pub fn valid_syntax(mut self, value: bool) -> Self {
        self.valid_syntax = Some(value);
        self
    }

    pub fn domain(mut self, value: BulkEmailValidateResponseEmailResponseItemDomain) -> Self {
        self.domain = Some(value);
        self
    }

    pub fn account(mut self, value: BulkEmailValidateResponseEmailResponseItemAccount) -> Self {
        self.account = Some(value);
        self
    }

    pub fn dns(mut self, value: BulkEmailValidateResponseEmailResponseItemDns) -> Self {
        self.dns = Some(value);
        self
    }

    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn address(mut self, value: BulkEmailValidateResponseEmailResponseItemAddress) -> Self {
        self.address = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkEmailValidateResponseEmailResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](BulkEmailValidateResponseEmailResponseItemBuilder::success)
    /// - [`email`](BulkEmailValidateResponseEmailResponseItemBuilder::email)
    /// - [`valid_email`](BulkEmailValidateResponseEmailResponseItemBuilder::valid_email)
    /// - [`valid_syntax`](BulkEmailValidateResponseEmailResponseItemBuilder::valid_syntax)
    /// - [`domain`](BulkEmailValidateResponseEmailResponseItemBuilder::domain)
    /// - [`account`](BulkEmailValidateResponseEmailResponseItemBuilder::account)
    /// - [`dns`](BulkEmailValidateResponseEmailResponseItemBuilder::dns)
    pub fn build(self) -> Result<BulkEmailValidateResponseEmailResponseItem, BuildError> {
        Ok(BulkEmailValidateResponseEmailResponseItem {
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
