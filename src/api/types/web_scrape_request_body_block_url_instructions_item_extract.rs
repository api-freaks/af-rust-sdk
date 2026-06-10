pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemExtract {
    /// Defines what data to extract and how to extract it.
    pub extract: Option<WebScrapeRequestBodyBlockUrlInstructionsItemExtractExtract>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemExtract {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlInstructionsItemExtractBuilder {
        <WebScrapeRequestBodyBlockUrlInstructionsItemExtractBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemExtractBuilder {
    extract: Option<WebScrapeRequestBodyBlockUrlInstructionsItemExtractExtract>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemExtractBuilder {
    pub fn extract(
        mut self,
        value: WebScrapeRequestBodyBlockUrlInstructionsItemExtractExtract,
    ) -> Self {
        self.extract = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemExtract`].
    pub fn build(self) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemExtract, BuildError> {
        Ok(WebScrapeRequestBodyBlockUrlInstructionsItemExtract {
            extract: self.extract,
        })
    }
}
