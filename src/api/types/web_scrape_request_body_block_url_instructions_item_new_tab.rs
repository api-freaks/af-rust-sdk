pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemNewTab {
    pub new_tab: Option<bool>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemNewTab {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlInstructionsItemNewTabBuilder {
        <WebScrapeRequestBodyBlockUrlInstructionsItemNewTabBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemNewTabBuilder {
    new_tab: Option<bool>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemNewTabBuilder {
    pub fn new_tab(mut self, value: bool) -> Self {
        self.new_tab = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemNewTab`].
    pub fn build(self) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemNewTab, BuildError> {
        Ok(WebScrapeRequestBodyBlockUrlInstructionsItemNewTab {
            new_tab: self.new_tab,
        })
    }
}
