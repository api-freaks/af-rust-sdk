pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValueClickButtonByValue {
    #[serde(default)]
    pub place: String,
    #[serde(default)]
    pub value: String,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValueClickButtonByValue {
    pub fn builder(
    ) -> WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValueClickButtonByValueBuilder
    {
        <WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValueClickButtonByValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValueClickButtonByValueBuilder {
    place: Option<String>,
    value: Option<String>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValueClickButtonByValueBuilder {
    pub fn place(mut self, value: impl Into<String>) -> Self {
        self.place = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValueClickButtonByValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`place`](WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValueClickButtonByValueBuilder::place)
    /// - [`value`](WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValueClickButtonByValueBuilder::value)
    pub fn build(
        self,
    ) -> Result<
        WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValueClickButtonByValue,
        BuildError,
    > {
        Ok(
            WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValueClickButtonByValue {
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
