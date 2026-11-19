pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkDisposableEmailCheckV2RequestEmailDataItem {
    /// The email address to check.
    #[serde(default)]
    pub email: String,
    /// Optional IP address to look up for location and security data, such as the IP the signup came from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

impl BulkDisposableEmailCheckV2RequestEmailDataItem {
    pub fn builder() -> BulkDisposableEmailCheckV2RequestEmailDataItemBuilder {
        <BulkDisposableEmailCheckV2RequestEmailDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDisposableEmailCheckV2RequestEmailDataItemBuilder {
    email: Option<String>,
    ip: Option<String>,
}

impl BulkDisposableEmailCheckV2RequestEmailDataItemBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkDisposableEmailCheckV2RequestEmailDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](BulkDisposableEmailCheckV2RequestEmailDataItemBuilder::email)
    pub fn build(self) -> Result<BulkDisposableEmailCheckV2RequestEmailDataItem, BuildError> {
        Ok(BulkDisposableEmailCheckV2RequestEmailDataItem {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            ip: self.ip,
        })
    }
}
