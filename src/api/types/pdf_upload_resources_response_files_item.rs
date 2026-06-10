pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PdfUploadResourcesResponseFilesItem {
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "fileId")]
    #[serde(default)]
    pub file_id: String,
}

impl PdfUploadResourcesResponseFilesItem {
    pub fn builder() -> PdfUploadResourcesResponseFilesItemBuilder {
        <PdfUploadResourcesResponseFilesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PdfUploadResourcesResponseFilesItemBuilder {
    file_name: Option<String>,
    file_id: Option<String>,
}

impl PdfUploadResourcesResponseFilesItemBuilder {
    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PdfUploadResourcesResponseFilesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_name`](PdfUploadResourcesResponseFilesItemBuilder::file_name)
    /// - [`file_id`](PdfUploadResourcesResponseFilesItemBuilder::file_id)
    pub fn build(self) -> Result<PdfUploadResourcesResponseFilesItem, BuildError> {
        Ok(PdfUploadResourcesResponseFilesItem {
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            file_id: self
                .file_id
                .ok_or_else(|| BuildError::missing_field("file_id"))?,
        })
    }
}
