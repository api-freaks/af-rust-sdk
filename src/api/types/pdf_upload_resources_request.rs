pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PdfUploadResourcesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<Vec<Vec<u8>>>,
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
    #[serde(skip_serializing)]
    pub format: Option<PdfUploadResourcesRequestFormat>,
}
impl PdfUploadResourcesRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();

        if let Some(ref files) = self.file {
            for file_data in files {
                form = form.part(
                    "file",
                    reqwest::multipart::Part::bytes(file_data.clone())
                        .file_name("file")
                        .mime_str("application/octet-stream")
                        .unwrap(),
                );
            }
        }

        form
    }
}

impl PdfUploadResourcesRequest {
    pub fn builder() -> PdfUploadResourcesRequestBuilder {
        <PdfUploadResourcesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PdfUploadResourcesRequestBuilder {
    file: Option<Vec<Vec<u8>>>,
    api_key: Option<String>,
    format: Option<PdfUploadResourcesRequestFormat>,
}

impl PdfUploadResourcesRequestBuilder {
    pub fn file(mut self, value: Vec<Vec<u8>>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: PdfUploadResourcesRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PdfUploadResourcesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](PdfUploadResourcesRequestBuilder::api_key)
    pub fn build(self) -> Result<PdfUploadResourcesRequest, BuildError> {
        Ok(PdfUploadResourcesRequest {
            file: self.file,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
        })
    }
}
