pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkDisposableEmailCheckV2Request {
    /// Entries to check. Maximum 100 per request. Must not be empty, and must not contain null entries or entries without a valid email.
    #[serde(rename = "emailData")]
    #[serde(default)]
    pub email_data: Vec<BulkDisposableEmailCheckV2RequestEmailDataItem>,
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(skip)]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip)]
    pub format: Option<BulkDisposableEmailCheckV2RequestFormat>,
}

impl BulkDisposableEmailCheckV2Request {
    pub fn builder() -> BulkDisposableEmailCheckV2RequestBuilder {
        <BulkDisposableEmailCheckV2RequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDisposableEmailCheckV2RequestBuilder {
    email_data: Option<Vec<BulkDisposableEmailCheckV2RequestEmailDataItem>>,
    api_key: Option<String>,
    format: Option<BulkDisposableEmailCheckV2RequestFormat>,
}

impl BulkDisposableEmailCheckV2RequestBuilder {
    pub fn email_data(
        mut self,
        value: Vec<BulkDisposableEmailCheckV2RequestEmailDataItem>,
    ) -> Self {
        self.email_data = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: BulkDisposableEmailCheckV2RequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkDisposableEmailCheckV2Request`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email_data`](BulkDisposableEmailCheckV2RequestBuilder::email_data)
    /// - [`api_key`](BulkDisposableEmailCheckV2RequestBuilder::api_key)
    pub fn build(self) -> Result<BulkDisposableEmailCheckV2Request, BuildError> {
        Ok(BulkDisposableEmailCheckV2Request {
            email_data: self
                .email_data
                .ok_or_else(|| BuildError::missing_field("email_data"))?,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
        })
    }
}
