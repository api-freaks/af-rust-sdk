pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PdfUploadResourcesResponse {
    #[serde(default)]
    pub files: Vec<PdfUploadResourcesResponseFilesItem>,
}

impl PdfUploadResourcesResponse {
    pub fn builder() -> PdfUploadResourcesResponseBuilder {
        <PdfUploadResourcesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PdfUploadResourcesResponseBuilder {
    files: Option<Vec<PdfUploadResourcesResponseFilesItem>>,
}

impl PdfUploadResourcesResponseBuilder {
    pub fn files(mut self, value: Vec<PdfUploadResourcesResponseFilesItem>) -> Self {
        self.files = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PdfUploadResourcesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`files`](PdfUploadResourcesResponseBuilder::files)
    pub fn build(self) -> Result<PdfUploadResourcesResponse, BuildError> {
        Ok(PdfUploadResourcesResponse {
            files: self
                .files
                .ok_or_else(|| BuildError::missing_field("files"))?,
        })
    }
}
