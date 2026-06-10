pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PdfRestrictResponse {
    #[serde(rename = "taskId")]
    #[serde(default)]
    pub task_id: String,
    #[serde(rename = "inputIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_ids: Option<Vec<String>>,
}

impl PdfRestrictResponse {
    pub fn builder() -> PdfRestrictResponseBuilder {
        <PdfRestrictResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PdfRestrictResponseBuilder {
    task_id: Option<String>,
    input_ids: Option<Vec<String>>,
}

impl PdfRestrictResponseBuilder {
    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    pub fn input_ids(mut self, value: Vec<String>) -> Self {
        self.input_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PdfRestrictResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`task_id`](PdfRestrictResponseBuilder::task_id)
    pub fn build(self) -> Result<PdfRestrictResponse, BuildError> {
        Ok(PdfRestrictResponse {
            task_id: self
                .task_id
                .ok_or_else(|| BuildError::missing_field("task_id"))?,
            input_ids: self.input_ids,
        })
    }
}
