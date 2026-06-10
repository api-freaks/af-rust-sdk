pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkEmailValidateRequestEmailDataItem {
    /// Email address to validate
    #[serde(default)]
    pub email: String,
    /// Name associated with the email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// IP address associated with the email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

impl BulkEmailValidateRequestEmailDataItem {
    pub fn builder() -> BulkEmailValidateRequestEmailDataItemBuilder {
        <BulkEmailValidateRequestEmailDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkEmailValidateRequestEmailDataItemBuilder {
    email: Option<String>,
    name: Option<String>,
    ip: Option<String>,
}

impl BulkEmailValidateRequestEmailDataItemBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkEmailValidateRequestEmailDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](BulkEmailValidateRequestEmailDataItemBuilder::email)
    pub fn build(self) -> Result<BulkEmailValidateRequestEmailDataItem, BuildError> {
        Ok(BulkEmailValidateRequestEmailDataItem {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            name: self.name,
            ip: self.ip,
        })
    }
}
