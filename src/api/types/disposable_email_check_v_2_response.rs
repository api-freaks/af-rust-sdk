pub use crate::prelude::*;

/// Disposable email check response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisposableEmailCheckV2Response {
    /// The email address submitted for the check.
    #[serde(default)]
    pub email: String,
    /// Validates email address format compliance with RFC 5322 standards. When false, domain only contains name/validDomain and account/dns are omitted entirely.
    #[serde(rename = "validSyntax")]
    #[serde(default)]
    pub valid_syntax: bool,
    /// Domain-level disposable/spam/free classification for the email address.
    #[serde(default)]
    pub domain: DisposableEmailCheckV2ResponseDomain,
    /// Account-level detection details. Omitted entirely when validSyntax is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<DisposableEmailCheckV2ResponseAccount>,
    /// DNS records for the email domain. Omitted entirely when validSyntax is false or the domain does not exist (NXDOMAIN).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<DisposableEmailCheckV2ResponseDns>,
    /// The IP address that was looked up.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Geolocation and security enrichment data for the IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<DisposableEmailCheckV2ResponseAddress>,
}

impl DisposableEmailCheckV2Response {
    pub fn builder() -> DisposableEmailCheckV2ResponseBuilder {
        <DisposableEmailCheckV2ResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisposableEmailCheckV2ResponseBuilder {
    email: Option<String>,
    valid_syntax: Option<bool>,
    domain: Option<DisposableEmailCheckV2ResponseDomain>,
    account: Option<DisposableEmailCheckV2ResponseAccount>,
    dns: Option<DisposableEmailCheckV2ResponseDns>,
    ip: Option<String>,
    address: Option<DisposableEmailCheckV2ResponseAddress>,
}

impl DisposableEmailCheckV2ResponseBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn valid_syntax(mut self, value: bool) -> Self {
        self.valid_syntax = Some(value);
        self
    }

    pub fn domain(mut self, value: DisposableEmailCheckV2ResponseDomain) -> Self {
        self.domain = Some(value);
        self
    }

    pub fn account(mut self, value: DisposableEmailCheckV2ResponseAccount) -> Self {
        self.account = Some(value);
        self
    }

    pub fn dns(mut self, value: DisposableEmailCheckV2ResponseDns) -> Self {
        self.dns = Some(value);
        self
    }

    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn address(mut self, value: DisposableEmailCheckV2ResponseAddress) -> Self {
        self.address = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DisposableEmailCheckV2Response`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](DisposableEmailCheckV2ResponseBuilder::email)
    /// - [`valid_syntax`](DisposableEmailCheckV2ResponseBuilder::valid_syntax)
    /// - [`domain`](DisposableEmailCheckV2ResponseBuilder::domain)
    pub fn build(self) -> Result<DisposableEmailCheckV2Response, BuildError> {
        Ok(DisposableEmailCheckV2Response {
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
        })
    }
}
