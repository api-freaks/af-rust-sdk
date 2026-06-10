pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PdfMergeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<Vec<Vec<u8>>>,
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
    #[serde(skip_serializing)]
    pub format: Option<PdfMergeRequestFormat>,
    #[serde(skip_serializing)]
    #[serde(default)]
    pub file_id: Vec<Option<String>>,
    #[serde(skip_serializing)]
    pub destroy: Option<bool>,
    #[serde(skip_serializing)]
    pub output: Option<String>,
    #[serde(skip_serializing)]
    pub webhook_url: Option<String>,
    #[serde(skip_serializing)]
    pub webhook_failure_notification: Option<bool>,
}
impl PdfMergeRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();

        if let Some(ref files) = self.file {
            for file_data in files {
                form = form.part(
                    "file",
                    reqwest::multipart::Part::bytes(file_data.clone())
                        .file_name("file")
                        .mime_str("application/octet-stream")
                        .unwrap(),
                );
            }
        }

        form
    }
}

impl PdfMergeRequest {
    pub fn builder() -> PdfMergeRequestBuilder {
        <PdfMergeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PdfMergeRequestBuilder {
    file: Option<Vec<Vec<u8>>>,
    api_key: Option<String>,
    format: Option<PdfMergeRequestFormat>,
    file_id: Option<Vec<Option<String>>>,
    destroy: Option<bool>,
    output: Option<String>,
    webhook_url: Option<String>,
    webhook_failure_notification: Option<bool>,
}

impl PdfMergeRequestBuilder {
    pub fn file(mut self, value: Vec<Vec<u8>>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: PdfMergeRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn file_id(mut self, value: Vec<Option<String>>) -> Self {
        self.file_id = Some(value);
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

    pub fn webhook_url(mut self, value: impl Into<String>) -> Self {
        self.webhook_url = Some(value.into());
        self
    }

    pub fn webhook_failure_notification(mut self, value: bool) -> Self {
        self.webhook_failure_notification = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PdfMergeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](PdfMergeRequestBuilder::api_key)
    /// - [`file_id`](PdfMergeRequestBuilder::file_id)
    pub fn build(self) -> Result<PdfMergeRequest, BuildError> {
        Ok(PdfMergeRequest {
            file: self.file,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            file_id: self
                .file_id
                .ok_or_else(|| BuildError::missing_field("file_id"))?,
            destroy: self.destroy,
            output: self.output,
            webhook_url: self.webhook_url,
            webhook_failure_notification: self.webhook_failure_notification,
        })
    }
}
