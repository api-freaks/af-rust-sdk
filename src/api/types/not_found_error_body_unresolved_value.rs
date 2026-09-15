pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NotFoundErrorBodyUnresolvedValue {
    /// Detail message explaining why this symbol was not resolved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Optional list of supported symbols that closely match the unresolved symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestions: Option<Vec<String>>,
}

impl NotFoundErrorBodyUnresolvedValue {
    pub fn builder() -> NotFoundErrorBodyUnresolvedValueBuilder {
        <NotFoundErrorBodyUnresolvedValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotFoundErrorBodyUnresolvedValueBuilder {
    message: Option<String>,
    suggestions: Option<Vec<String>>,
}

impl NotFoundErrorBodyUnresolvedValueBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn suggestions(mut self, value: Vec<String>) -> Self {
        self.suggestions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NotFoundErrorBodyUnresolvedValue`].
    pub fn build(self) -> Result<NotFoundErrorBodyUnresolvedValue, BuildError> {
        Ok(NotFoundErrorBodyUnresolvedValue {
            message: self.message,
            suggestions: self.suggestions,
        })
    }
}
