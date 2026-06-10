pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PdfConvertToJpgRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes::option")]
    pub file: Option<Vec<u8>>,
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
    #[serde(skip_serializing)]
    pub format: Option<PdfConvertToJpgRequestFormat>,
    #[serde(skip_serializing)]
    pub file_id: Option<String>,
    #[serde(skip_serializing)]
    pub destroy: Option<bool>,
    #[serde(skip_serializing)]
    pub output: Option<String>,
    #[serde(skip_serializing)]
    pub quality: Option<i64>,
    #[serde(skip_serializing)]
    pub pages: Option<String>,
    #[serde(skip_serializing)]
    pub resolution: Option<i64>,
    #[serde(skip_serializing)]
    pub image_smoothing: Option<String>,
    #[serde(skip_serializing)]
    pub profile: Option<PdfConvertToJpgRequestProfile>,
    #[serde(skip_serializing)]
    pub webhook_url: Option<String>,
    #[serde(skip_serializing)]
    pub webhook_failure_notification: Option<bool>,
}
impl PdfConvertToJpgRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();

        if let Some(ref file_data) = self.file {
            form = form.part(
                "file",
                reqwest::multipart::Part::bytes(file_data.clone())
                    .file_name("file")
                    .mime_str("application/octet-stream")
                    .unwrap(),
            );
        }

        form
    }
}

impl PdfConvertToJpgRequest {
    pub fn builder() -> PdfConvertToJpgRequestBuilder {
        <PdfConvertToJpgRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PdfConvertToJpgRequestBuilder {
    file: Option<Vec<u8>>,
    api_key: Option<String>,
    format: Option<PdfConvertToJpgRequestFormat>,
    file_id: Option<String>,
    destroy: Option<bool>,
    output: Option<String>,
    quality: Option<i64>,
    pages: Option<String>,
    resolution: Option<i64>,
    image_smoothing: Option<String>,
    profile: Option<PdfConvertToJpgRequestProfile>,
    webhook_url: Option<String>,
    webhook_failure_notification: Option<bool>,
}

impl PdfConvertToJpgRequestBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: PdfConvertToJpgRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn destroy(mut self, value: bool) -> Self {
        self.destroy = Some(value);
        self
    }

    pub fn output(mut self, value: impl Into<String>) -> Self {
        self.output = Some(value.into());
        self
    }

    pub fn quality(mut self, value: i64) -> Self {
        self.quality = Some(value);
        self
    }

    pub fn pages(mut self, value: impl Into<String>) -> Self {
        self.pages = Some(value.into());
        self
    }

    pub fn resolution(mut self, value: i64) -> Self {
        self.resolution = Some(value);
        self
    }

    pub fn image_smoothing(mut self, value: impl Into<String>) -> Self {
        self.image_smoothing = Some(value.into());
        self
    }

    pub fn profile(mut self, value: PdfConvertToJpgRequestProfile) -> Self {
        self.profile = Some(value);
        self
    }

    pub fn webhook_url(mut self, value: impl Into<String>) -> Self {
        self.webhook_url = Some(value.into());
        self
    }

    pub fn webhook_failure_notification(mut self, value: bool) -> Self {
        self.webhook_failure_notification = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PdfConvertToJpgRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](PdfConvertToJpgRequestBuilder::api_key)
    pub fn build(self) -> Result<PdfConvertToJpgRequest, BuildError> {
        Ok(PdfConvertToJpgRequest {
            file: self.file,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            file_id: self.file_id,
            destroy: self.destroy,
            output: self.output,
            quality: self.quality,
            pages: self.pages,
            resolution: self.resolution,
            image_smoothing: self.image_smoothing,
            profile: self.profile,
            webhook_url: self.webhook_url,
            webhook_failure_notification: self.webhook_failure_notification,
        })
    }
}
