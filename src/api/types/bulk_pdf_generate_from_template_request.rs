pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BulkPdfGenerateFromTemplateRequest {
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes")]
    pub file: Vec<u8>,
    #[serde(rename = "apiKey")]
    #[serde(skip)]
    #[serde(default)]
    pub api_key: String,
    #[serde(skip)]
    #[serde(default)]
    pub template_id: String,
    #[serde(skip)]
    pub version: Option<String>,
}
impl BulkPdfGenerateFromTemplateRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();

        form = form.part(
            "file",
            reqwest::multipart::Part::bytes(self.file.clone())
                .file_name(crate::core::upload_file_name(&self.file))
                .mime_str(crate::core::upload_mime_type(&self.file))
                .unwrap(),
        );

        form
    }
}

impl BulkPdfGenerateFromTemplateRequest {
    pub fn builder() -> BulkPdfGenerateFromTemplateRequestBuilder {
        <BulkPdfGenerateFromTemplateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkPdfGenerateFromTemplateRequestBuilder {
    file: Option<Vec<u8>>,
    api_key: Option<String>,
    template_id: Option<String>,
    version: Option<String>,
}

impl BulkPdfGenerateFromTemplateRequestBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn template_id(mut self, value: impl Into<String>) -> Self {
        self.template_id = Some(value.into());
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkPdfGenerateFromTemplateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](BulkPdfGenerateFromTemplateRequestBuilder::file)
    /// - [`api_key`](BulkPdfGenerateFromTemplateRequestBuilder::api_key)
    /// - [`template_id`](BulkPdfGenerateFromTemplateRequestBuilder::template_id)
    pub fn build(self) -> Result<BulkPdfGenerateFromTemplateRequest, BuildError> {
        Ok(BulkPdfGenerateFromTemplateRequest {
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            template_id: self
                .template_id
                .ok_or_else(|| BuildError::missing_field("template_id"))?,
            version: self.version,
        })
    }
}
