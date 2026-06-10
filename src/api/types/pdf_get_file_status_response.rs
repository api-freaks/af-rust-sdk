pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PdfGetFileStatusResponse {
    /// The unique identifier of the file.
    #[serde(rename = "fileId")]
    #[serde(default)]
    pub file_id: String,
    /// The name of the file.
    #[serde(rename = "fileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// The type of the file (e.g., 'pdf').
    #[serde(rename = "fileType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    /// The timestamp when the file was created.
    #[serde(rename = "fileCreationTime")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub file_creation_time: DateTime<FixedOffset>,
    /// Date on which the file is scheduled to be deleted, in UTC.
    #[serde(rename = "fileDeletionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_deletion_time: Option<NaiveDate>,
}

impl PdfGetFileStatusResponse {
    pub fn builder() -> PdfGetFileStatusResponseBuilder {
        <PdfGetFileStatusResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PdfGetFileStatusResponseBuilder {
    file_id: Option<String>,
    file_name: Option<String>,
    file_type: Option<String>,
    file_creation_time: Option<DateTime<FixedOffset>>,
    file_deletion_time: Option<NaiveDate>,
}

impl PdfGetFileStatusResponseBuilder {
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

    /// Consumes the builder and constructs a [`PdfGetFileStatusResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_id`](PdfGetFileStatusResponseBuilder::file_id)
    /// - [`file_creation_time`](PdfGetFileStatusResponseBuilder::file_creation_time)
    pub fn build(self) -> Result<PdfGetFileStatusResponse, BuildError> {
        Ok(PdfGetFileStatusResponse {
            file_id: self
                .file_id
                .ok_or_else(|| BuildError::missing_field("file_id"))?,
            file_name: self.file_name,
            file_type: self.file_type,
            file_creation_time: self
                .file_creation_time
                .ok_or_else(|| BuildError::missing_field("file_creation_time"))?,
            file_deletion_time: self.file_deletion_time,
        })
    }
}
