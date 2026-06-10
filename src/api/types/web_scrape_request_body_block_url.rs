pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WebScrapeRequestBodyBlockUrl {
    /// List of script or URL patterns to block during network requests.
    #[serde(rename = "blockUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_url: Option<Vec<String>>,
    /// List of cookies to be set in the browser session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<Vec<WebScrapeRequestBodyBlockUrlCookiesItem>>,
    /// An ordered list of step-by-step scraping instructions to be executed in the browser.
    #[serde(default)]
    pub instructions: Vec<WebScrapeRequestBodyBlockUrlInstructionsItem>,
}

impl WebScrapeRequestBodyBlockUrl {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlBuilder {
        <WebScrapeRequestBodyBlockUrlBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlBuilder {
    block_url: Option<Vec<String>>,
    cookies: Option<Vec<WebScrapeRequestBodyBlockUrlCookiesItem>>,
    instructions: Option<Vec<WebScrapeRequestBodyBlockUrlInstructionsItem>>,
}

impl WebScrapeRequestBodyBlockUrlBuilder {
    pub fn block_url(mut self, value: Vec<String>) -> Self {
        self.block_url = Some(value);
        self
    }

    pub fn cookies(mut self, value: Vec<WebScrapeRequestBodyBlockUrlCookiesItem>) -> Self {
        self.cookies = Some(value);
        self
    }

    pub fn instructions(
        mut self,
        value: Vec<WebScrapeRequestBodyBlockUrlInstructionsItem>,
    ) -> Self {
        self.instructions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrl`].
    /// This method will fail if any of the following fields are not set:
    /// - [`instructions`](WebScrapeRequestBodyBlockUrlBuilder::instructions)
    pub fn build(self) -> Result<WebScrapeRequestBodyBlockUrl, BuildError> {
        Ok(WebScrapeRequestBodyBlockUrl {
            block_url: self.block_url,
            cookies: self.cookies,
            instructions: self
                .instructions
                .ok_or_else(|| BuildError::missing_field("instructions"))?,
        })
    }
}
