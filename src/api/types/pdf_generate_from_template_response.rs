pub use crate::prelude::*;

/// Result of a PDF generation request, containing the hosted URL of the generated PDF.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PdfGenerateFromTemplateResponse {
    /// The template that was rendered.
    #[serde(default)]
    pub template_id: String,
    /// Hosted URL of the generated PDF.
    #[serde(default)]
    pub pdf_url: String,
    /// When the PDF was generated (ISO 8601).
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// When the hosted PDF expires (ISO 8601).
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expiration_time: DateTime<FixedOffset>,
}

impl PdfGenerateFromTemplateResponse {
    pub fn builder() -> PdfGenerateFromTemplateResponseBuilder {
        <PdfGenerateFromTemplateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PdfGenerateFromTemplateResponseBuilder {
    template_id: Option<String>,
    pdf_url: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    expiration_time: Option<DateTime<FixedOffset>>,
}

impl PdfGenerateFromTemplateResponseBuilder {
    pub fn template_id(mut self, value: impl Into<String>) -> Self {
        self.template_id = Some(value.into());
        self
    }

    pub fn pdf_url(mut self, value: impl Into<String>) -> Self {
        self.pdf_url = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn expiration_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expiration_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PdfGenerateFromTemplateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`template_id`](PdfGenerateFromTemplateResponseBuilder::template_id)
    /// - [`pdf_url`](PdfGenerateFromTemplateResponseBuilder::pdf_url)
    /// - [`created_at`](PdfGenerateFromTemplateResponseBuilder::created_at)
    /// - [`expiration_time`](PdfGenerateFromTemplateResponseBuilder::expiration_time)
    pub fn build(self) -> Result<PdfGenerateFromTemplateResponse, BuildError> {
        Ok(PdfGenerateFromTemplateResponse {
            template_id: self
                .template_id
                .ok_or_else(|| BuildError::missing_field("template_id"))?,
            pdf_url: self
                .pdf_url
                .ok_or_else(|| BuildError::missing_field("pdf_url"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            expiration_time: self
                .expiration_time
                .ok_or_else(|| BuildError::missing_field("expiration_time"))?,
        })
    }
}
