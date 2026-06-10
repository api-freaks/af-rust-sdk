pub use crate::prelude::*;

/// Defines what data to extract and how to extract it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemExtractExtract {
    /// CSS selector or XPath to extract HTML content. Example: "/html/body"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    /// CSS selector or XPath to extract text content. Example: "/html/body/div/div[2]/text"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// CSS selector or XPath to extract user data. Example: "/html/body/div/div[2]"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemExtractExtract {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlInstructionsItemExtractExtractBuilder {
        <WebScrapeRequestBodyBlockUrlInstructionsItemExtractExtractBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemExtractExtractBuilder {
    html: Option<String>,
    text: Option<String>,
    user_data: Option<String>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemExtractExtractBuilder {
    pub fn html(mut self, value: impl Into<String>) -> Self {
        self.html = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn user_data(mut self, value: impl Into<String>) -> Self {
        self.user_data = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemExtractExtract`].
    pub fn build(
        self,
    ) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemExtractExtract, BuildError> {
        Ok(WebScrapeRequestBodyBlockUrlInstructionsItemExtractExtract {
            html: self.html,
            text: self.text,
            user_data: self.user_data,
        })
    }
}
