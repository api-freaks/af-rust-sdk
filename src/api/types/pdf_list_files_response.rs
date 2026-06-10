pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PdfListFilesResponse {
    #[serde(default)]
    pub files: Vec<PdfListFilesResponseFilesItem>,
}

impl PdfListFilesResponse {
    pub fn builder() -> PdfListFilesResponseBuilder {
        <PdfListFilesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PdfListFilesResponseBuilder {
    files: Option<Vec<PdfListFilesResponseFilesItem>>,
}

impl PdfListFilesResponseBuilder {
    pub fn files(mut self, value: Vec<PdfListFilesResponseFilesItem>) -> Self {
        self.files = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PdfListFilesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`files`](PdfListFilesResponseBuilder::files)
    pub fn build(self) -> Result<PdfListFilesResponse, BuildError> {
        Ok(PdfListFilesResponse {
            files: self
                .files
                .ok_or_else(|| BuildError::missing_field("files"))?,
        })
    }
}
