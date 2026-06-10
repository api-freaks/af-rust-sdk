pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PdfRestrictRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes::option")]
    pub file: Option<Vec<u8>>,
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
    #[serde(skip_serializing)]
    pub format: Option<PdfRestrictRequestFormat>,
    #[serde(skip_serializing)]
    pub file_id: Option<String>,
    #[serde(skip_serializing)]
    pub destroy: Option<bool>,
    #[serde(skip_serializing)]
    pub output: Option<String>,
    #[serde(skip_serializing)]
    pub file_password: Option<String>,
    #[serde(skip_serializing)]
    #[serde(default)]
    pub user_password: String,
    #[serde(skip_serializing)]
    pub owner_password: Option<String>,
    #[serde(skip_serializing)]
    #[serde(default)]
    pub restrictions: Vec<Option<PdfRestrictRequestRestrictionsItem>>,
    #[serde(skip_serializing)]
    pub webhook_url: Option<String>,
    #[serde(skip_serializing)]
    pub webhook_failure_notification: Option<bool>,
}
impl PdfRestrictRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();

        if let Some(ref file_data) = self.file {
            form = form.part(
                "file",
                reqwest::multipart::Part::bytes(file_data.clone())
                    .file_name("file")
                    .mime_str("application/octet-stream")
                    .unwrap(),
            );
        }

        form
    }
}

impl PdfRestrictRequest {
    pub fn builder() -> PdfRestrictRequestBuilder {
        <PdfRestrictRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PdfRestrictRequestBuilder {
    file: Option<Vec<u8>>,
    api_key: Option<String>,
    format: Option<PdfRestrictRequestFormat>,
    file_id: Option<String>,
    destroy: Option<bool>,
    output: Option<String>,
    file_password: Option<String>,
    user_password: Option<String>,
    owner_password: Option<String>,
    restrictions: Option<Vec<Option<PdfRestrictRequestRestrictionsItem>>>,
    webhook_url: Option<String>,
    webhook_failure_notification: Option<bool>,
}

impl PdfRestrictRequestBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: PdfRestrictRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn destroy(mut self, value: bool) -> Self {
        self.destroy = Some(value);
        self
    }

    pub fn output(mut self, value: impl Into<String>) -> Self {
        self.output = Some(value.into());
        self
    }

    pub fn file_password(mut self, value: impl Into<String>) -> Self {
        self.file_password = Some(value.into());
        self
    }

    pub fn user_password(mut self, value: impl Into<String>) -> Self {
        self.user_password = Some(value.into());
        self
    }

    pub fn owner_password(mut self, value: impl Into<String>) -> Self {
        self.owner_password = Some(value.into());
        self
    }

    pub fn restrictions(mut self, value: Vec<Option<PdfRestrictRequestRestrictionsItem>>) -> Self {
        self.restrictions = Some(value);
        self
    }

    pub fn webhook_url(mut self, value: impl Into<String>) -> Self {
        self.webhook_url = Some(value.into());
        self
    }

    pub fn webhook_failure_notification(mut self, value: bool) -> Self {
        self.webhook_failure_notification = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PdfRestrictRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](PdfRestrictRequestBuilder::api_key)
    /// - [`user_password`](PdfRestrictRequestBuilder::user_password)
    /// - [`restrictions`](PdfRestrictRequestBuilder::restrictions)
    pub fn build(self) -> Result<PdfRestrictRequest, BuildError> {
        Ok(PdfRestrictRequest {
            file: self.file,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            file_id: self.file_id,
            destroy: self.destroy,
            output: self.output,
            file_password: self.file_password,
            user_password: self
                .user_password
                .ok_or_else(|| BuildError::missing_field("user_password"))?,
            owner_password: self.owner_password,
            restrictions: self
                .restrictions
                .ok_or_else(|| BuildError::missing_field("restrictions"))?,
            webhook_url: self.webhook_url,
            webhook_failure_notification: self.webhook_failure_notification,
        })
    }
}
