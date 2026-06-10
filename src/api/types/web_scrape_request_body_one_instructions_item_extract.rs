pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct WebScrapeRequestBodyOneInstructionsItemExtract {
    /// Defines what data to extract and how to extract it.
    pub extract: Option<WebScrapeRequestBodyOneInstructionsItemExtractExtract>,
}

impl WebScrapeRequestBodyOneInstructionsItemExtract {
    pub fn builder() -> WebScrapeRequestBodyOneInstructionsItemExtractBuilder {
        <WebScrapeRequestBodyOneInstructionsItemExtractBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyOneInstructionsItemExtractBuilder {
    extract: Option<WebScrapeRequestBodyOneInstructionsItemExtractExtract>,
}

impl WebScrapeRequestBodyOneInstructionsItemExtractBuilder {
    pub fn extract(mut self, value: WebScrapeRequestBodyOneInstructionsItemExtractExtract) -> Self {
        self.extract = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyOneInstructionsItemExtract`].
    pub fn build(self) -> Result<WebScrapeRequestBodyOneInstructionsItemExtract, BuildError> {
        Ok(WebScrapeRequestBodyOneInstructionsItemExtract {
            extract: self.extract,
        })
    }
}
