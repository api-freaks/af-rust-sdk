pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkEmailValidateRequest {
    /// Array of email objects for bulk validation
    #[serde(rename = "emailData")]
    #[serde(default)]
    pub email_data: Vec<BulkEmailValidateRequestEmailDataItem>,
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response
    #[serde(skip_serializing)]
    pub format: Option<BulkEmailValidateRequestFormat>,
}

impl BulkEmailValidateRequest {
    pub fn builder() -> BulkEmailValidateRequestBuilder {
        <BulkEmailValidateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkEmailValidateRequestBuilder {
    email_data: Option<Vec<BulkEmailValidateRequestEmailDataItem>>,
    api_key: Option<String>,
    format: Option<BulkEmailValidateRequestFormat>,
}

impl BulkEmailValidateRequestBuilder {
    pub fn email_data(mut self, value: Vec<BulkEmailValidateRequestEmailDataItem>) -> Self {
        self.email_data = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: BulkEmailValidateRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkEmailValidateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email_data`](BulkEmailValidateRequestBuilder::email_data)
    /// - [`api_key`](BulkEmailValidateRequestBuilder::api_key)
    pub fn build(self) -> Result<BulkEmailValidateRequest, BuildError> {
        Ok(BulkEmailValidateRequest {
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
