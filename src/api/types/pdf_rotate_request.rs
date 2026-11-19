pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PdfRotateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes::option")]
    pub file: Option<Vec<u8>>,
    #[serde(rename = "apiKey")]
    #[serde(skip)]
    #[serde(default)]
    pub api_key: String,
    #[serde(skip)]
    pub format: Option<PdfRotateRequestFormat>,
    #[serde(skip)]
    pub file_id: Option<String>,
    #[serde(skip)]
    pub destroy: Option<bool>,
    #[serde(skip)]
    pub output: Option<String>,
    #[serde(skip)]
    pub pages: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub rotate: i64,
    #[serde(skip)]
    pub webhook_url: Option<String>,
    #[serde(skip)]
    pub webhook_failure_notification: Option<bool>,
    /// Value sent in the `X-Webhook-Authorization` header of the webhook call to `webhook_url`, so your endpoint can verify the request.
    #[serde(skip)]
    pub webhook_authorization: Option<String>,
}
impl PdfRotateRequest {
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

impl PdfRotateRequest {
    pub fn builder() -> PdfRotateRequestBuilder {
        <PdfRotateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PdfRotateRequestBuilder {
    file: Option<Vec<u8>>,
    api_key: Option<String>,
    format: Option<PdfRotateRequestFormat>,
    file_id: Option<String>,
    destroy: Option<bool>,
    output: Option<String>,
    pages: Option<String>,
    rotate: Option<i64>,
    webhook_url: Option<String>,
    webhook_failure_notification: Option<bool>,
    webhook_authorization: Option<String>,
}

impl PdfRotateRequestBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: PdfRotateRequestFormat) -> Self {
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

    pub fn pages(mut self, value: impl Into<String>) -> Self {
        self.pages = Some(value.into());
        self
    }

    pub fn rotate(mut self, value: i64) -> Self {
        self.rotate = Some(value);
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

    /// Consumes the builder and constructs a [`PdfRotateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](PdfRotateRequestBuilder::api_key)
    /// - [`rotate`](PdfRotateRequestBuilder::rotate)
    pub fn build(self) -> Result<PdfRotateRequest, BuildError> {
        Ok(PdfRotateRequest {
            file: self.file,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            file_id: self.file_id,
            destroy: self.destroy,
            output: self.output,
            pages: self.pages,
            rotate: self
                .rotate
                .ok_or_else(|| BuildError::missing_field("rotate"))?,
            webhook_url: self.webhook_url,
            webhook_failure_notification: self.webhook_failure_notification,
            webhook_authorization: self.webhook_authorization,
        })
    }
}
