pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PdfSplitRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes::option")]
    pub file: Option<Vec<u8>>,
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
    #[serde(skip_serializing)]
    pub format: Option<PdfSplitRequestFormat>,
    #[serde(skip_serializing)]
    pub file_id: Option<String>,
    #[serde(skip_serializing)]
    pub destroy: Option<bool>,
    #[serde(skip_serializing)]
    pub output: Option<String>,
    #[serde(skip_serializing)]
    #[serde(default)]
    pub pages: Vec<Option<String>>,
    #[serde(skip_serializing)]
    pub webhook_url: Option<String>,
    #[serde(skip_serializing)]
    pub webhook_failure_notification: Option<bool>,
}
impl PdfSplitRequest {
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

impl PdfSplitRequest {
    pub fn builder() -> PdfSplitRequestBuilder {
        <PdfSplitRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PdfSplitRequestBuilder {
    file: Option<Vec<u8>>,
    api_key: Option<String>,
    format: Option<PdfSplitRequestFormat>,
    file_id: Option<String>,
    destroy: Option<bool>,
    output: Option<String>,
    pages: Option<Vec<Option<String>>>,
    webhook_url: Option<String>,
    webhook_failure_notification: Option<bool>,
}

impl PdfSplitRequestBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: PdfSplitRequestFormat) -> Self {
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

    pub fn pages(mut self, value: Vec<Option<String>>) -> Self {
        self.pages = Some(value);
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

    /// Consumes the builder and constructs a [`PdfSplitRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](PdfSplitRequestBuilder::api_key)
    /// - [`pages`](PdfSplitRequestBuilder::pages)
    pub fn build(self) -> Result<PdfSplitRequest, BuildError> {
        Ok(PdfSplitRequest {
            file: self.file,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            file_id: self.file_id,
            destroy: self.destroy,
            output: self.output,
            pages: self
                .pages
                .ok_or_else(|| BuildError::missing_field("pages"))?,
            webhook_url: self.webhook_url,
            webhook_failure_notification: self.webhook_failure_notification,
        })
    }
}
