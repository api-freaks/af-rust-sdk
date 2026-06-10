pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PdfCompressResponse {
    /// Task id of the requested task which can be used to check the task status
    #[serde(rename = "taskId")]
    #[serde(default)]
    pub task_id: String,
    /// File Ids for the input files provided for the task. Not given with destroy param.
    #[serde(rename = "inputIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_ids: Option<Vec<String>>,
}

impl PdfCompressResponse {
    pub fn builder() -> PdfCompressResponseBuilder {
        <PdfCompressResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PdfCompressResponseBuilder {
    task_id: Option<String>,
    input_ids: Option<Vec<String>>,
}

impl PdfCompressResponseBuilder {
    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    pub fn input_ids(mut self, value: Vec<String>) -> Self {
        self.input_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PdfCompressResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`task_id`](PdfCompressResponseBuilder::task_id)
    pub fn build(self) -> Result<PdfCompressResponse, BuildError> {
        Ok(PdfCompressResponse {
            task_id: self
                .task_id
                .ok_or_else(|| BuildError::missing_field("task_id"))?,
            input_ids: self.input_ids,
        })
    }
}
