pub use crate::prelude::*;

/// Defines what data to extract and how to extract it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebScrapeRequestBodyOneInstructionsItemExtractExtract {
    /// CSS selector or XPath to extract HTML content. Example: "/html/body"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    /// CSS selector or XPath to extract text content. Example: "/html/body/div/div[2]/text"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl WebScrapeRequestBodyOneInstructionsItemExtractExtract {
    pub fn builder() -> WebScrapeRequestBodyOneInstructionsItemExtractExtractBuilder {
        <WebScrapeRequestBodyOneInstructionsItemExtractExtractBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyOneInstructionsItemExtractExtractBuilder {
    html: Option<String>,
    text: Option<String>,
}

impl WebScrapeRequestBodyOneInstructionsItemExtractExtractBuilder {
    pub fn html(mut self, value: impl Into<String>) -> Self {
        self.html = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyOneInstructionsItemExtractExtract`].
    pub fn build(
        self,
    ) -> Result<WebScrapeRequestBodyOneInstructionsItemExtractExtract, BuildError> {
        Ok(WebScrapeRequestBodyOneInstructionsItemExtractExtract {
            html: self.html,
            text: self.text,
        })
    }
}
