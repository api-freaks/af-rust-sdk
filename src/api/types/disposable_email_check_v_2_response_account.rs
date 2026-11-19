pub use crate::prelude::*;

/// Account-level detection details. Omitted entirely when validSyntax is false.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisposableEmailCheckV2ResponseAccount {
    /// Detection flag identifying role-based email addresses (e.g., admin@, support@).
    #[serde(default)]
    pub role: bool,
}

impl DisposableEmailCheckV2ResponseAccount {
    pub fn builder() -> DisposableEmailCheckV2ResponseAccountBuilder {
        <DisposableEmailCheckV2ResponseAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisposableEmailCheckV2ResponseAccountBuilder {
    role: Option<bool>,
}

impl DisposableEmailCheckV2ResponseAccountBuilder {
    pub fn role(mut self, value: bool) -> Self {
        self.role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DisposableEmailCheckV2ResponseAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`role`](DisposableEmailCheckV2ResponseAccountBuilder::role)
    pub fn build(self) -> Result<DisposableEmailCheckV2ResponseAccount, BuildError> {
        Ok(DisposableEmailCheckV2ResponseAccount {
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
        })
    }
}
