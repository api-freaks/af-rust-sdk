pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GrammarDetectResponse {
    /// List of words flagged as grammatically incorrect
    #[serde(default)]
    pub grammar_errors: Vec<GrammarDetectResponseGrammarErrorsItem>,
}

impl GrammarDetectResponse {
    pub fn builder() -> GrammarDetectResponseBuilder {
        <GrammarDetectResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GrammarDetectResponseBuilder {
    grammar_errors: Option<Vec<GrammarDetectResponseGrammarErrorsItem>>,
}

impl GrammarDetectResponseBuilder {
    pub fn grammar_errors(mut self, value: Vec<GrammarDetectResponseGrammarErrorsItem>) -> Self {
        self.grammar_errors = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GrammarDetectResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`grammar_errors`](GrammarDetectResponseBuilder::grammar_errors)
    pub fn build(self) -> Result<GrammarDetectResponse, BuildError> {
        Ok(GrammarDetectResponse {
            grammar_errors: self
                .grammar_errors
                .ok_or_else(|| BuildError::missing_field("grammar_errors"))?,
        })
    }
}
