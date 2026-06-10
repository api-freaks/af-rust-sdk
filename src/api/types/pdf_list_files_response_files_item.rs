pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PdfListFilesResponseFilesItem {
    #[serde(rename = "fileId")]
    #[serde(default)]
    pub file_id: String,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "fileType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(rename = "fileCreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub file_creation_time: Option<DateTime<FixedOffset>>,
    #[serde(rename = "fileDeletionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_deletion_time: Option<NaiveDate>,
}

impl PdfListFilesResponseFilesItem {
    pub fn builder() -> PdfListFilesResponseFilesItemBuilder {
        <PdfListFilesResponseFilesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PdfListFilesResponseFilesItemBuilder {
    file_id: Option<String>,
    file_name: Option<String>,
    file_type: Option<String>,
    file_creation_time: Option<DateTime<FixedOffset>>,
    file_deletion_time: Option<NaiveDate>,
}

impl PdfListFilesResponseFilesItemBuilder {
    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn file_type(mut self, value: impl Into<String>) -> Self {
        self.file_type = Some(value.into());
        self
    }

    pub fn file_creation_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.file_creation_time = Some(value);
        self
    }

    pub fn file_deletion_time(mut self, value: NaiveDate) -> Self {
        self.file_deletion_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PdfListFilesResponseFilesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_id`](PdfListFilesResponseFilesItemBuilder::file_id)
    /// - [`file_name`](PdfListFilesResponseFilesItemBuilder::file_name)
    pub fn build(self) -> Result<PdfListFilesResponseFilesItem, BuildError> {
        Ok(PdfListFilesResponseFilesItem {
            file_id: self
                .file_id
                .ok_or_else(|| BuildError::missing_field("file_id"))?,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            file_type: self.file_type,
            file_creation_time: self.file_creation_time,
            file_deletion_time: self.file_deletion_time,
        })
    }
}
