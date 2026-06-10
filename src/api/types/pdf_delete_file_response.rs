pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PdfDeleteFileResponse {
    /// The unique identifier of the file.
    #[serde(rename = "fileId")]
    #[serde(default)]
    pub file_id: String,
    /// Indicates whether the file deletion request was successful.
    #[serde(default)]
    pub success: bool,
    /// A message indicating the result of the file deletion request.
    #[serde(default)]
    pub message: String,
}

impl PdfDeleteFileResponse {
    pub fn builder() -> PdfDeleteFileResponseBuilder {
        <PdfDeleteFileResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PdfDeleteFileResponseBuilder {
    file_id: Option<String>,
    success: Option<bool>,
    message: Option<String>,
}

impl PdfDeleteFileResponseBuilder {
    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PdfDeleteFileResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_id`](PdfDeleteFileResponseBuilder::file_id)
    /// - [`success`](PdfDeleteFileResponseBuilder::success)
    /// - [`message`](PdfDeleteFileResponseBuilder::message)
    pub fn build(self) -> Result<PdfDeleteFileResponse, BuildError> {
        Ok(PdfDeleteFileResponse {
            file_id: self
                .file_id
                .ok_or_else(|| BuildError::missing_field("file_id"))?,
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
