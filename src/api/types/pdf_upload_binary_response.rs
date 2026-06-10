pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PdfUploadBinaryResponse {
    /// The name of the uploaded file.
    #[serde(rename = "fileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// The unique identifier assigned to the uploaded file.
    #[serde(rename = "fileId")]
    #[serde(default)]
    pub file_id: String,
}

impl PdfUploadBinaryResponse {
    pub fn builder() -> PdfUploadBinaryResponseBuilder {
        <PdfUploadBinaryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PdfUploadBinaryResponseBuilder {
    file_name: Option<String>,
    file_id: Option<String>,
}

impl PdfUploadBinaryResponseBuilder {
    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PdfUploadBinaryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_id`](PdfUploadBinaryResponseBuilder::file_id)
    pub fn build(self) -> Result<PdfUploadBinaryResponse, BuildError> {
        Ok(PdfUploadBinaryResponse {
            file_name: self.file_name,
            file_id: self
                .file_id
                .ok_or_else(|| BuildError::missing_field("file_id"))?,
        })
    }
}
