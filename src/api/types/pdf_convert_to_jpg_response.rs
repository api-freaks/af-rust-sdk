pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PdfConvertToJpgResponse {
    #[serde(rename = "taskId")]
    #[serde(default)]
    pub task_id: String,
    #[serde(rename = "inputIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_ids: Option<Vec<String>>,
}

impl PdfConvertToJpgResponse {
    pub fn builder() -> PdfConvertToJpgResponseBuilder {
        <PdfConvertToJpgResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PdfConvertToJpgResponseBuilder {
    task_id: Option<String>,
    input_ids: Option<Vec<String>>,
}

impl PdfConvertToJpgResponseBuilder {
    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    pub fn input_ids(mut self, value: Vec<String>) -> Self {
        self.input_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PdfConvertToJpgResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`task_id`](PdfConvertToJpgResponseBuilder::task_id)
    pub fn build(self) -> Result<PdfConvertToJpgResponse, BuildError> {
        Ok(PdfConvertToJpgResponse {
            task_id: self
                .task_id
                .ok_or_else(|| BuildError::missing_field("task_id"))?,
            input_ids: self.input_ids,
        })
    }
}
