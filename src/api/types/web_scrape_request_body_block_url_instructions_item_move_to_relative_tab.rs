pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemMoveToRelativeTab {
    pub move_to_relative_tab: Option<i64>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemMoveToRelativeTab {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlInstructionsItemMoveToRelativeTabBuilder {
        <WebScrapeRequestBodyBlockUrlInstructionsItemMoveToRelativeTabBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemMoveToRelativeTabBuilder {
    move_to_relative_tab: Option<i64>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemMoveToRelativeTabBuilder {
    pub fn move_to_relative_tab(mut self, value: i64) -> Self {
        self.move_to_relative_tab = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemMoveToRelativeTab`].
    pub fn build(
        self,
    ) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemMoveToRelativeTab, BuildError> {
        Ok(
            WebScrapeRequestBodyBlockUrlInstructionsItemMoveToRelativeTab {
                move_to_relative_tab: self.move_to_relative_tab,
            },
        )
    }
}
