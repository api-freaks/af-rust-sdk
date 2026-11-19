pub use crate::prelude::*;

/// PDF output options. Applies when file_type is PDF.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkScreenshotCaptureRequestUrlsItemPdfOptions {
    /// Paper size of the PDF. Applies when file_type is PDF. Without it, width and height are used if set, otherwise the page renders as one long page: A0, A1, A2, A3, A4, A5, A6, Letter, Legal, Tabloid, or Ledger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<BulkScreenshotCaptureRequestUrlsItemPdfOptionsFormat>,
    /// Renders the PDF in landscape orientation, swapping the width and height of the paper size. Applies when file_type is PDF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landscape: Option<bool>,
    /// Pages to keep in the PDF, as start-end, for example 1-3 or 5-5. Pages are rendered first and then trimmed. Has no effect on a single-page PDF. Applies when file_type is PDF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_range: Option<String>,
    /// CSS media type used to render the PDF: print or screen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<BulkScreenshotCaptureRequestUrlsItemPdfOptionsMedia>,
    /// Includes background colors and images in the PDF. Applies when file_type is PDF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_background: Option<bool>,
}

impl BulkScreenshotCaptureRequestUrlsItemPdfOptions {
    pub fn builder() -> BulkScreenshotCaptureRequestUrlsItemPdfOptionsBuilder {
        <BulkScreenshotCaptureRequestUrlsItemPdfOptionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkScreenshotCaptureRequestUrlsItemPdfOptionsBuilder {
    format: Option<BulkScreenshotCaptureRequestUrlsItemPdfOptionsFormat>,
    landscape: Option<bool>,
    page_range: Option<String>,
    media: Option<BulkScreenshotCaptureRequestUrlsItemPdfOptionsMedia>,
    print_background: Option<bool>,
}

impl BulkScreenshotCaptureRequestUrlsItemPdfOptionsBuilder {
    pub fn format(mut self, value: BulkScreenshotCaptureRequestUrlsItemPdfOptionsFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn landscape(mut self, value: bool) -> Self {
        self.landscape = Some(value);
        self
    }

    pub fn page_range(mut self, value: impl Into<String>) -> Self {
        self.page_range = Some(value.into());
        self
    }

    pub fn media(mut self, value: BulkScreenshotCaptureRequestUrlsItemPdfOptionsMedia) -> Self {
        self.media = Some(value);
        self
    }

    pub fn print_background(mut self, value: bool) -> Self {
        self.print_background = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkScreenshotCaptureRequestUrlsItemPdfOptions`].
    pub fn build(self) -> Result<BulkScreenshotCaptureRequestUrlsItemPdfOptions, BuildError> {
        Ok(BulkScreenshotCaptureRequestUrlsItemPdfOptions {
            format: self.format,
            landscape: self.landscape,
            page_range: self.page_range,
            media: self.media,
            print_background: self.print_background,
        })
    }
}
