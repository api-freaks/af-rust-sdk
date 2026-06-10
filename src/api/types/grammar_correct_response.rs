pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GrammarCorrectResponse {
    /// The fully corrected text
    #[serde(default)]
    pub grammar_correction: String,
}

impl GrammarCorrectResponse {
    pub fn builder() -> GrammarCorrectResponseBuilder {
        <GrammarCorrectResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GrammarCorrectResponseBuilder {
    grammar_correction: Option<String>,
}

impl GrammarCorrectResponseBuilder {
    pub fn grammar_correction(mut self, value: impl Into<String>) -> Self {
        self.grammar_correction = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GrammarCorrectResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`grammar_correction`](GrammarCorrectResponseBuilder::grammar_correction)
    pub fn build(self) -> Result<GrammarCorrectResponse, BuildError> {
        Ok(GrammarCorrectResponse {
            grammar_correction: self
                .grammar_correction
                .ok_or_else(|| BuildError::missing_field("grammar_correction"))?,
        })
    }
}
