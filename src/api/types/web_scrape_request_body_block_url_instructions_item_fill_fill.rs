pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemFillFill {
    #[serde(default)]
    pub place: String,
    #[serde(default)]
    pub value: String,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemFillFill {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlInstructionsItemFillFillBuilder {
        <WebScrapeRequestBodyBlockUrlInstructionsItemFillFillBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemFillFillBuilder {
    place: Option<String>,
    value: Option<String>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemFillFillBuilder {
    pub fn place(mut self, value: impl Into<String>) -> Self {
        self.place = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemFillFill`].
    /// This method will fail if any of the following fields are not set:
    /// - [`place`](WebScrapeRequestBodyBlockUrlInstructionsItemFillFillBuilder::place)
    /// - [`value`](WebScrapeRequestBodyBlockUrlInstructionsItemFillFillBuilder::value)
    pub fn build(self) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemFillFill, BuildError> {
        Ok(WebScrapeRequestBodyBlockUrlInstructionsItemFillFill {
            place: self
                .place
                .ok_or_else(|| BuildError::missing_field("place"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
