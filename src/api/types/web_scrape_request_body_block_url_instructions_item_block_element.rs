pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemBlockElement {
    /// List of CSS selectors or XPaths for elements to block or hide on the page.
    pub block_element: Option<Vec<String>>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemBlockElement {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlInstructionsItemBlockElementBuilder {
        <WebScrapeRequestBodyBlockUrlInstructionsItemBlockElementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemBlockElementBuilder {
    block_element: Option<Vec<String>>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemBlockElementBuilder {
    pub fn block_element(mut self, value: Vec<String>) -> Self {
        self.block_element = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemBlockElement`].
    pub fn build(
        self,
    ) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemBlockElement, BuildError> {
        Ok(WebScrapeRequestBodyBlockUrlInstructionsItemBlockElement {
            block_element: self.block_element,
        })
    }
}
