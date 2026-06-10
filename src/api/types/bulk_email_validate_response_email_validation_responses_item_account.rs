pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkEmailValidateResponseEmailValidationResponsesItemAccount {
    #[serde(default)]
    pub role: bool,
    #[serde(rename = "fullMailBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_mail_box: Option<bool>,
}

impl BulkEmailValidateResponseEmailValidationResponsesItemAccount {
    pub fn builder() -> BulkEmailValidateResponseEmailValidationResponsesItemAccountBuilder {
        <BulkEmailValidateResponseEmailValidationResponsesItemAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkEmailValidateResponseEmailValidationResponsesItemAccountBuilder {
    role: Option<bool>,
    full_mail_box: Option<bool>,
}

impl BulkEmailValidateResponseEmailValidationResponsesItemAccountBuilder {
    pub fn role(mut self, value: bool) -> Self {
        self.role = Some(value);
        self
    }

    pub fn full_mail_box(mut self, value: bool) -> Self {
        self.full_mail_box = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkEmailValidateResponseEmailValidationResponsesItemAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`role`](BulkEmailValidateResponseEmailValidationResponsesItemAccountBuilder::role)
    pub fn build(
        self,
    ) -> Result<BulkEmailValidateResponseEmailValidationResponsesItemAccount, BuildError> {
        Ok(
            BulkEmailValidateResponseEmailValidationResponsesItemAccount {
                role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
                full_mail_box: self.full_mail_box,
            },
        )
    }
}
