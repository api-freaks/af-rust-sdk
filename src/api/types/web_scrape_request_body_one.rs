pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
// required fields kept strict for untagged-union matching (fix_rust_untagged_union_required.py)
pub struct WebScrapeRequestBodyOne {
    /// A list of sequential instructions to execute against the static HTML response. Each item is one StaticInstruction object, with exactly one action field set.
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
