pub use crate::prelude::*;

/// A selector/value pair used by fill, select, and clickButtonByValue instructions.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemSelect {
    /// CSS or XPath selector identifying the target element.
    #[serde(default)]
    pub place: String,
    /// Value to fill, select, or match against.
    #[serde(default)]
    pub value: String,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemSelect {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlInstructionsItemSelectBuilder {
        <WebScrapeRequestBodyBlockUrlInstructionsItemSelectBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemSelectBuilder {
    place: Option<String>,
    value: Option<String>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemSelectBuilder {
    pub fn place(mut self, value: impl Into<String>) -> Self {
        self.place = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemSelect`].
    /// This method will fail if any of the following fields are not set:
    /// - [`place`](WebScrapeRequestBodyBlockUrlInstructionsItemSelectBuilder::place)
    /// - [`value`](WebScrapeRequestBodyBlockUrlInstructionsItemSelectBuilder::value)
    pub fn build(self) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemSelect, BuildError> {
        Ok(WebScrapeRequestBodyBlockUrlInstructionsItemSelect {
            place: self
                .place
                .ok_or_else(|| BuildError::missing_field("place"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
