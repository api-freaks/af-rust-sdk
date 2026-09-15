pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkEmailValidateResponseEmailResponseItemAccount {
    #[serde(default)]
    pub role: bool,
    #[serde(rename = "fullMailBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_mail_box: Option<bool>,
}

impl BulkEmailValidateResponseEmailResponseItemAccount {
    pub fn builder() -> BulkEmailValidateResponseEmailResponseItemAccountBuilder {
        <BulkEmailValidateResponseEmailResponseItemAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkEmailValidateResponseEmailResponseItemAccountBuilder {
    role: Option<bool>,
    full_mail_box: Option<bool>,
}

impl BulkEmailValidateResponseEmailResponseItemAccountBuilder {
    pub fn role(mut self, value: bool) -> Self {
        self.role = Some(value);
        self
    }

    pub fn full_mail_box(mut self, value: bool) -> Self {
        self.full_mail_box = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkEmailValidateResponseEmailResponseItemAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`role`](BulkEmailValidateResponseEmailResponseItemAccountBuilder::role)
    pub fn build(self) -> Result<BulkEmailValidateResponseEmailResponseItemAccount, BuildError> {
        Ok(BulkEmailValidateResponseEmailResponseItemAccount {
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            full_mail_box: self.full_mail_box,
        })
    }
}
