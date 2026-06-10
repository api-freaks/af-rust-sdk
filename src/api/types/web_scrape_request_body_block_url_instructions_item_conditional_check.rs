pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(transparent)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemConditionalCheck {
    pub conditional_check: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemConditionalCheck {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlInstructionsItemConditionalCheckBuilder {
        <WebScrapeRequestBodyBlockUrlInstructionsItemConditionalCheckBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemConditionalCheckBuilder {
    conditional_check: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemConditionalCheckBuilder {
    pub fn conditional_check(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.conditional_check = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemConditionalCheck`].
    pub fn build(
        self,
    ) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemConditionalCheck, BuildError> {
        Ok(
            WebScrapeRequestBodyBlockUrlInstructionsItemConditionalCheck {
                conditional_check: self.conditional_check,
            },
        )
    }
}
