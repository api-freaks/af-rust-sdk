pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct WebScrapeRequestBodyOne {
    /// A list of sequential instructions to execute after receiving the static HTML response. Supported methods include `postForm`, `getForm`, and `extract`.
    pub instructions: Vec<WebScrapeRequestBodyOneInstructionsItem>,
}

impl WebScrapeRequestBodyOne {
    pub fn builder() -> WebScrapeRequestBodyOneBuilder {
        <WebScrapeRequestBodyOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyOneBuilder {
    instructions: Option<Vec<WebScrapeRequestBodyOneInstructionsItem>>,
}

impl WebScrapeRequestBodyOneBuilder {
    pub fn instructions(mut self, value: Vec<WebScrapeRequestBodyOneInstructionsItem>) -> Self {
        self.instructions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`instructions`](WebScrapeRequestBodyOneBuilder::instructions)
    pub fn build(self) -> Result<WebScrapeRequestBodyOne, BuildError> {
        Ok(WebScrapeRequestBodyOne {
            instructions: self
                .instructions
                .ok_or_else(|| BuildError::missing_field("instructions"))?,
        })
    }
}
