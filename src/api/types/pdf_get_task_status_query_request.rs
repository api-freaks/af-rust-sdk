pub use crate::prelude::*;

/// Query parameters for pdf_get_task_status
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PdfGetTaskStatusQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<PdfGetTaskStatusRequestFormat>,
    /// The unique ID of the PDF processing task for which the status is requested.
    #[serde(default)]
    pub task_id: String,
}

impl PdfGetTaskStatusQueryRequest {
    pub fn builder() -> PdfGetTaskStatusQueryRequestBuilder {
        <PdfGetTaskStatusQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PdfGetTaskStatusQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<PdfGetTaskStatusRequestFormat>,
    task_id: Option<String>,
}

impl PdfGetTaskStatusQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: PdfGetTaskStatusRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PdfGetTaskStatusQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](PdfGetTaskStatusQueryRequestBuilder::api_key)
    /// - [`task_id`](PdfGetTaskStatusQueryRequestBuilder::task_id)
    pub fn build(self) -> Result<PdfGetTaskStatusQueryRequest, BuildError> {
        Ok(PdfGetTaskStatusQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            task_id: self
                .task_id
                .ok_or_else(|| BuildError::missing_field("task_id"))?,
        })
    }
}
