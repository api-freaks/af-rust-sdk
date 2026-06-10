pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PdfGetTaskStatusResponse {
    /// The unique identifier of the PDF processing task.
    #[serde(rename = "taskId")]
    #[serde(default)]
    pub task_id: String,
    /// The current status of the task (e.g., 'queued', 'processing', 'completed', 'failed').
    #[serde(default)]
    pub status: String,
    /// The timestamp when the task status was created, formatted as 'yyyy-MM-dd HH:mm:ss'.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The URL to download all output files as a single ZIP archive. This is present only when the task status is 'COMPLETED'.
    #[serde(rename = "zipOutputUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_output_url: Option<String>,
    /// The unique identifier for the ZIP file. This is present only when the task status is 'COMPLETED'.
    #[serde(rename = "zipFileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_file_id: Option<String>,
    /// A list of URLs for each individual output file. This is present only when the task is 'COMPLETED'.
    #[serde(rename = "outputUrls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_urls: Option<Vec<String>>,
    /// A list of unique IDs for the output files. This is present only when the task is 'COMPLETED'.
    #[serde(rename = "outputIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_ids: Option<Vec<String>>,
    /// A list of unique IDs for the input files submitted with the task. If the task was initiated with the `destroy` parameter set to `true`, `inputIds` will not be generated.
    #[serde(rename = "inputIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_ids: Option<Vec<String>>,
    /// The error code for the PDF Task. This is included only if the task fails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// A descriptive message indicating the reason for task failure. This is included only if the task fails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The timestamp when the task status will expire and be removed from the system, formatted as 'yyyy-MM-dd HH:mm:ss'.
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub expires_at: Option<DateTime<FixedOffset>>,
}

impl PdfGetTaskStatusResponse {
    pub fn builder() -> PdfGetTaskStatusResponseBuilder {
        <PdfGetTaskStatusResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PdfGetTaskStatusResponseBuilder {
    task_id: Option<String>,
    status: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    zip_output_url: Option<String>,
    zip_file_id: Option<String>,
    output_urls: Option<Vec<String>>,
    output_ids: Option<Vec<String>>,
    input_ids: Option<Vec<String>>,
    error: Option<String>,
    message: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
}

impl PdfGetTaskStatusResponseBuilder {
    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn zip_output_url(mut self, value: impl Into<String>) -> Self {
        self.zip_output_url = Some(value.into());
        self
    }

    pub fn zip_file_id(mut self, value: impl Into<String>) -> Self {
        self.zip_file_id = Some(value.into());
        self
    }

    pub fn output_urls(mut self, value: Vec<String>) -> Self {
        self.output_urls = Some(value);
        self
    }

    pub fn output_ids(mut self, value: Vec<String>) -> Self {
        self.output_ids = Some(value);
        self
    }

    pub fn input_ids(mut self, value: Vec<String>) -> Self {
        self.input_ids = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PdfGetTaskStatusResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`task_id`](PdfGetTaskStatusResponseBuilder::task_id)
    /// - [`status`](PdfGetTaskStatusResponseBuilder::status)
    /// - [`created_at`](PdfGetTaskStatusResponseBuilder::created_at)
    pub fn build(self) -> Result<PdfGetTaskStatusResponse, BuildError> {
        Ok(PdfGetTaskStatusResponse {
            task_id: self
                .task_id
                .ok_or_else(|| BuildError::missing_field("task_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            zip_output_url: self.zip_output_url,
            zip_file_id: self.zip_file_id,
            output_urls: self.output_urls,
            output_ids: self.output_ids,
            input_ids: self.input_ids,
            error: self.error,
            message: self.message,
            expires_at: self.expires_at,
        })
    }
}
