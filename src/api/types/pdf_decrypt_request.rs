pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PdfDecryptRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes::option")]
    pub file: Option<Vec<u8>>,
    #[serde(rename = "apiKey")]
    #[serde(skip)]
    #[serde(default)]
    pub api_key: String,
    #[serde(skip)]
    pub format: Option<PdfDecryptRequestFormat>,
    #[serde(skip)]
    pub file_id: Option<String>,
    #[serde(skip)]
    pub destroy: Option<bool>,
    #[serde(skip)]
    pub output: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub file_password: String,
    #[serde(skip)]
    pub webhook_url: Option<String>,
    #[serde(skip)]
    pub webhook_failure_notification: Option<bool>,
    /// Value sent in the `X-Webhook-Authorization` header of the webhook call to `webhook_url`, so your endpoint can verify the request.
    #[serde(skip)]
    pub webhook_authorization: Option<String>,
}
impl PdfDecryptRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();

        if let Some(ref file_data) = self.file {
            form = form.part(
                "file",
                reqwest::multipart::Part::bytes(file_data.clone())
                    .file_name(crate::core::upload_file_name(&file_data))
                    .mime_str(crate::core::upload_mime_type(&file_data))
                    .unwrap(),
            );
        }

        form
    }
}

impl PdfDecryptRequest {
    pub fn builder() -> PdfDecryptRequestBuilder {
        <PdfDecryptRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PdfDecryptRequestBuilder {
    file: Option<Vec<u8>>,
    api_key: Option<String>,
    format: Option<PdfDecryptRequestFormat>,
    file_id: Option<String>,
    destroy: Option<bool>,
    output: Option<String>,
    file_password: Option<String>,
    webhook_url: Option<String>,
    webhook_failure_notification: Option<bool>,
    webhook_authorization: Option<String>,
}

impl PdfDecryptRequestBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: PdfDecryptRequestFormat) -> Self {
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

    pub fn webhook_url(mut self, value: impl Into<String>) -> Self {
        self.webhook_url = Some(value.into());
        self
    }

    pub fn webhook_failure_notification(mut self, value: bool) -> Self {
        self.webhook_failure_notification = Some(value);
        self
    }

    /// Sets the `X-Webhook-Authorization` header.
    pub fn webhook_authorization(mut self, value: impl Into<String>) -> Self {
        self.webhook_authorization = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PdfDecryptRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](PdfDecryptRequestBuilder::api_key)
    /// - [`file_password`](PdfDecryptRequestBuilder::file_password)
    pub fn build(self) -> Result<PdfDecryptRequest, BuildError> {
        Ok(PdfDecryptRequest {
            file: self.file,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            file_id: self.file_id,
            destroy: self.destroy,
            output: self.output,
            file_password: self
                .file_password
                .ok_or_else(|| BuildError::missing_field("file_password"))?,
            webhook_url: self.webhook_url,
            webhook_failure_notification: self.webhook_failure_notification,
            webhook_authorization: self.webhook_authorization,
        })
    }
}
