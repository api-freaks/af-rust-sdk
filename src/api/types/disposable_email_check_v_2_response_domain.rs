pub use crate::prelude::*;

/// Domain-level disposable/spam/free classification for the email address.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisposableEmailCheckV2ResponseDomain {
    /// Extracted domain component from the email address.
    #[serde(default)]
    pub name: String,
    /// Domain validation status confirming DNS resolution capability.
    #[serde(rename = "validDomain")]
    #[serde(default)]
    pub valid_domain: bool,
    /// Classification flag identifying domains from temporary/throwaway email service providers. Omitted when validSyntax is false or the domain could not be resolved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disposable: Option<bool>,
    /// Classification indicating free or consumer-grade email service providers. Omitted when validSyntax is false or the domain could not be resolved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free: Option<bool>,
    /// Reputation indicator flagging domains on spam databases. Omitted when validSyntax is false or the domain could not be resolved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spam: Option<bool>,
}

impl DisposableEmailCheckV2ResponseDomain {
    pub fn builder() -> DisposableEmailCheckV2ResponseDomainBuilder {
        <DisposableEmailCheckV2ResponseDomainBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisposableEmailCheckV2ResponseDomainBuilder {
    name: Option<String>,
    valid_domain: Option<bool>,
    disposable: Option<bool>,
    free: Option<bool>,
    spam: Option<bool>,
}

impl DisposableEmailCheckV2ResponseDomainBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn valid_domain(mut self, value: bool) -> Self {
        self.valid_domain = Some(value);
        self
    }

    pub fn disposable(mut self, value: bool) -> Self {
        self.disposable = Some(value);
        self
    }

    pub fn free(mut self, value: bool) -> Self {
        self.free = Some(value);
        self
    }

    pub fn spam(mut self, value: bool) -> Self {
        self.spam = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DisposableEmailCheckV2ResponseDomain`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](DisposableEmailCheckV2ResponseDomainBuilder::name)
    /// - [`valid_domain`](DisposableEmailCheckV2ResponseDomainBuilder::valid_domain)
    pub fn build(self) -> Result<DisposableEmailCheckV2ResponseDomain, BuildError> {
        Ok(DisposableEmailCheckV2ResponseDomain {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            valid_domain: self
                .valid_domain
                .ok_or_else(|| BuildError::missing_field("valid_domain"))?,
            disposable: self.disposable,
            free: self.free,
            spam: self.spam,
        })
    }
}
