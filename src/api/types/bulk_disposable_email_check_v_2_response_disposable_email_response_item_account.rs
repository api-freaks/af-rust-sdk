pub use crate::prelude::*;

/// Account-level detection details. Omitted entirely when validSyntax is false.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAccount {
    /// Detection flag identifying role-based email addresses (e.g., admin@, support@).
    #[serde(default)]
    pub role: bool,
}

impl BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAccount {
    pub fn builder() -> BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAccountBuilder
    {
        <BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAccountBuilder {
    role: Option<bool>,
}

impl BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAccountBuilder {
    pub fn role(mut self, value: bool) -> Self {
        self.role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`role`](BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAccountBuilder::role)
    pub fn build(
        self,
    ) -> Result<BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAccount, BuildError>
    {
        Ok(
            BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItemAccount {
                role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            },
        )
    }
}
