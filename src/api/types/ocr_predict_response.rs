pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OcrPredictResponse {
    /// Array containing extracted text. Structure varies based on input type and new_line parameter:
    /// - Single file, new_line=0: Array with single string element
    /// - Single file, new_line=1: Array of strings (one per line)
    /// - Bulk/ZIP file, new_line=0: Array of strings (one per file)
    /// - Bulk/ZIP file, new_line=1: Array of arrays (each inner array contains lines for respective file)
    #[serde(rename = "OCRText")]
    pub ocr_text: OcrPredictResponseOcrText,
}

impl OcrPredictResponse {
    pub fn builder() -> OcrPredictResponseBuilder {
        <OcrPredictResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OcrPredictResponseBuilder {
    ocr_text: Option<OcrPredictResponseOcrText>,
}

impl OcrPredictResponseBuilder {
    pub fn ocr_text(mut self, value: OcrPredictResponseOcrText) -> Self {
        self.ocr_text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OcrPredictResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ocr_text`](OcrPredictResponseBuilder::ocr_text)
    pub fn build(self) -> Result<OcrPredictResponse, BuildError> {
        Ok(OcrPredictResponse {
            ocr_text: self
                .ocr_text
                .ok_or_else(|| BuildError::missing_field("ocr_text"))?,
        })
    }
}
