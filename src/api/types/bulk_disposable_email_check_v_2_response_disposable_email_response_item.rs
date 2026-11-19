pub use crate::prelude::*;

/// Disposable email check result for a single item in the bulk request.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItem {
    /// The email address submitted for the check.
    #[serde(default)]
    pub email: String,
    /// Validates email address format compliance with RFC 5322 standards. When false, domain only contains name/validDomain and account/dns are omitted entirely.
    #[serde(rename = "validSyntax")]
    #[serde(default)]
    pub valid_syntax: bool,
    /// Domain-level disposable/spam/free classification for the email address.
    #[serde(default)]
    pub domain: BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemDomain,
    /// Account-level detection details. Omitted entirely when validSyntax is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAccount>,
    /// DNS records for the email domain. Omitted entirely when validSyntax is false or the domain does not exist (NXDOMAIN).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemDns>,
    /// The IP address that was looked up.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Geolocation and security enrichment data for the IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddress>,
}

impl BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItem {
    pub fn builder() -> BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemBuilder {
        <BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemBuilder {
    email: Option<String>,
    valid_syntax: Option<bool>,
    domain: Option<BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemDomain>,
    account: Option<BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAccount>,
    dns: Option<BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemDns>,
    ip: Option<String>,
    address: Option<BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddress>,
}

impl BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn valid_syntax(mut self, value: bool) -> Self {
        self.valid_syntax = Some(value);
        self
    }

    pub fn domain(
        mut self,
        value: BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemDomain,
    ) -> Self {
        self.domain = Some(value);
        self
    }

    pub fn account(
        mut self,
        value: BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAccount,
    ) -> Self {
        self.account = Some(value);
        self
    }

    pub fn dns(
        mut self,
        value: BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemDns,
    ) -> Self {
        self.dns = Some(value);
        self
    }

    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn address(
        mut self,
        value: BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAddress,
    ) -> Self {
        self.address = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemBuilder::email)
    /// - [`valid_syntax`](BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemBuilder::valid_syntax)
    /// - [`domain`](BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemBuilder::domain)
    pub fn build(
        self,
    ) -> Result<BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItem, BuildError> {
        Ok(
            BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItem {
                email: self
                    .email
                    .ok_or_else(|| BuildError::missing_field("email"))?,
                valid_syntax: self
                    .valid_syntax
                    .ok_or_else(|| BuildError::missing_field("valid_syntax"))?,
                domain: self
                    .domain
                    .ok_or_else(|| BuildError::missing_field("domain"))?,
                account: self.account,
                dns: self.dns,
                ip: self.ip,
                address: self.address,
            },
        )
    }
}
