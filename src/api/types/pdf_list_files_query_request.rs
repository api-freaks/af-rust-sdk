pub use crate::prelude::*;

/// Query parameters for pdf_list_files
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PdfListFilesQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Specifies the desired format for the API response. Choose 'json' for a JSON object or 'xml' for an XML structure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<PdfListFilesRequestFormat>,
}

impl PdfListFilesQueryRequest {
    pub fn builder() -> PdfListFilesQueryRequestBuilder {
        <PdfListFilesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PdfListFilesQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<PdfListFilesRequestFormat>,
}

impl PdfListFilesQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: PdfListFilesRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PdfListFilesQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](PdfListFilesQueryRequestBuilder::api_key)
    pub fn build(self) -> Result<PdfListFilesQueryRequest, BuildError> {
        Ok(PdfListFilesQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
        })
    }
}
