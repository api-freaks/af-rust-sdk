pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemSelect {
    pub select: Option<WebScrapeRequestBodyBlockUrlInstructionsItemSelectSelect>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemSelect {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlInstructionsItemSelectBuilder {
        <WebScrapeRequestBodyBlockUrlInstructionsItemSelectBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemSelectBuilder {
    select: Option<WebScrapeRequestBodyBlockUrlInstructionsItemSelectSelect>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemSelectBuilder {
    pub fn select(
        mut self,
        value: WebScrapeRequestBodyBlockUrlInstructionsItemSelectSelect,
    ) -> Self {
        self.select = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemSelect`].
    pub fn build(self) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemSelect, BuildError> {
        Ok(WebScrapeRequestBodyBlockUrlInstructionsItemSelect {
            select: self.select,
        })
    }
}
