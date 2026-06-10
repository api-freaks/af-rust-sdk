pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemJsExe {
    pub js_exe: Option<String>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemJsExe {
    pub fn builder() -> WebScrapeRequestBodyBlockUrlInstructionsItemJsExeBuilder {
        <WebScrapeRequestBodyBlockUrlInstructionsItemJsExeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebScrapeRequestBodyBlockUrlInstructionsItemJsExeBuilder {
    js_exe: Option<String>,
}

impl WebScrapeRequestBodyBlockUrlInstructionsItemJsExeBuilder {
    pub fn js_exe(mut self, value: impl Into<String>) -> Self {
        self.js_exe = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebScrapeRequestBodyBlockUrlInstructionsItemJsExe`].
    pub fn build(self) -> Result<WebScrapeRequestBodyBlockUrlInstructionsItemJsExe, BuildError> {
        Ok(WebScrapeRequestBodyBlockUrlInstructionsItemJsExe {
            js_exe: self.js_exe,
        })
    }
}
