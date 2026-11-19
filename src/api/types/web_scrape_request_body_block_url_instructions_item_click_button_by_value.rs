pub use crate::prelude::*;

/// A selector/value pair used by fill, select, and clickButtonByValue instructions.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValue {
    /// CSS or XPath selector identifying the target element.
    #[serde(default)]
    pub place: String,
    /// Value to fill, select, or match against.
    #[serde(default)]
    pub value: String,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValue {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValueBuilder {
        <WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValueBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValueBuilder {
    place: Option<String>,
    value: Option<String>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValueBuilder {
    pub fn place(mut self, value: impl Into<String>) -> Self {
        self.place = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`place`](WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValueBuilder::place)
    /// - [`value`](WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValueBuilder::value)
    pub fn build(
        self,
    ) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValue, BuildError> {
        Ok(
            WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValue {
                place: self
                    .place
                    .ok_or_else(|| BuildError::missing_field("place"))?,
                value: self
                    .value
                    .ok_or_else(|| BuildError::missing_field("value"))?,
            },
        )
    }
}
