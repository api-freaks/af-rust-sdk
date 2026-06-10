pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemSelectSelect {
    #[serde(default)]
    pub place: String,
    #[serde(default)]
    pub value: String,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemSelectSelect {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlInstructionsItemSelectSelectBuilder {
        <WebScrapeRequestBodyBlockUrlInstructionsItemSelectSelectBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemSelectSelectBuilder {
    place: Option<String>,
    value: Option<String>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemSelectSelectBuilder {
    pub fn place(mut self, value: impl Into<String>) -> Self {
        self.place = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemSelectSelect`].
    /// This method will fail if any of the following fields are not set:
    /// - [`place`](WebScrapeRequestBodyBlockUrlInstructionsItemSelectSelectBuilder::place)
    /// - [`value`](WebScrapeRequestBodyBlockUrlInstructionsItemSelectSelectBuilder::value)
    pub fn build(
        self,
    ) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemSelectSelect, BuildError> {
        Ok(WebScrapeRequestBodyBlockUrlInstructionsItemSelectSelect {
            place: self
                .place
                .ok_or_else(|| BuildError::missing_field("place"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
