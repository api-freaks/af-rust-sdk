pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemScreenshot {
    /// Captures screenshot of the page.
    pub screenshot: Option<WebScrapeRequestBodyBlockUrlInstructionsItemScreenshotScreenshot>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemScreenshot {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlInstructionsItemScreenshotBuilder {
        <WebScrapeRequestBodyBlockUrlInstructionsItemScreenshotBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemScreenshotBuilder {
    screenshot: Option<WebScrapeRequestBodyBlockUrlInstructionsItemScreenshotScreenshot>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemScreenshotBuilder {
    pub fn screenshot(
        mut self,
        value: WebScrapeRequestBodyBlockUrlInstructionsItemScreenshotScreenshot,
    ) -> Self {
        self.screenshot = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemScreenshot`].
    pub fn build(
        self,
    ) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemScreenshot, BuildError> {
        Ok(WebScrapeRequestBodyBlockUrlInstructionsItemScreenshot {
            screenshot: self.screenshot,
        })
    }
}
