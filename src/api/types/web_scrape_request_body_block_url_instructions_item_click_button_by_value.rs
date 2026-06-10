pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValue {
    pub click_button_by_value:
        Option<WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValueClickButtonByValue>,
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
    click_button_by_value:
        Option<WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValueClickButtonByValue>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValueBuilder {
    pub fn click_button_by_value(
        mut self,
        value: WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValueClickButtonByValue,
    ) -> Self {
        self.click_button_by_value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValue`].
    pub fn build(
        self,
    ) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValue, BuildError> {
        Ok(
            WebScrapeRequestBodyBlockUrlInstructionsItemClickButtonByValue {
                click_button_by_value: self.click_button_by_value,
            },
        )
    }
}
