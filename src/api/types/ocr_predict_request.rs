pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OcrPredictRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes::option")]
    pub file: Option<Vec<u8>>,
    #[serde(rename = "apiKey")]
    #[serde(skip)]
    #[serde(default)]
    pub api_key: String,
    #[serde(skip_serializing)]
    pub model: OcrPredictRequestModel,
    #[serde(skip)]
    pub url: Option<String>,
    #[serde(skip)]
    pub page_range: Option<String>,
    #[serde(skip)]
    pub zone: Option<String>,
    #[serde(skip)]
    pub new_line: Option<i64>,
}
impl OcrPredictRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();

        if let Some(ref file_data) = self.file {
            form = form.part(
                "file",
                reqwest::multipart::Part::bytes(file_data.clone())
                    .file_name(crate::core::upload_file_name(&file_data))
                    .mime_str(crate::core::upload_mime_type(&file_data))
                    .unwrap(),
            );
        }

        form
    }
}

impl OcrPredictRequest {
    pub fn builder() -> OcrPredictRequestBuilder {
        <OcrPredictRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OcrPredictRequestBuilder {
    file: Option<Vec<u8>>,
    api_key: Option<String>,
    model: Option<OcrPredictRequestModel>,
    url: Option<String>,
    page_range: Option<String>,
    zone: Option<String>,
    new_line: Option<i64>,
}

impl OcrPredictRequestBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn model(mut self, value: OcrPredictRequestModel) -> Self {
        self.model = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn page_range(mut self, value: impl Into<String>) -> Self {
        self.page_range = Some(value.into());
        self
    }

    pub fn zone(mut self, value: impl Into<String>) -> Self {
        self.zone = Some(value.into());
        self
    }

    pub fn new_line(mut self, value: i64) -> Self {
        self.new_line = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OcrPredictRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](OcrPredictRequestBuilder::api_key)
    /// - [`model`](OcrPredictRequestBuilder::model)
    pub fn build(self) -> Result<OcrPredictRequest, BuildError> {
        Ok(OcrPredictRequest {
            file: self.file,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            url: self.url,
            page_range: self.page_range,
            zone: self.zone,
            new_line: self.new_line,
        })
    }
}
