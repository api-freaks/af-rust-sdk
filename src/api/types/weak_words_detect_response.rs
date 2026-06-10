pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WeakWordsDetectResponse {
    /// List of detected weak words
    #[serde(default)]
    pub weak_words: Vec<WeakWordsDetectResponseWeakWordsItem>,
}

impl WeakWordsDetectResponse {
    pub fn builder() -> WeakWordsDetectResponseBuilder {
        <WeakWordsDetectResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WeakWordsDetectResponseBuilder {
    weak_words: Option<Vec<WeakWordsDetectResponseWeakWordsItem>>,
}

impl WeakWordsDetectResponseBuilder {
    pub fn weak_words(mut self, value: Vec<WeakWordsDetectResponseWeakWordsItem>) -> Self {
        self.weak_words = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WeakWordsDetectResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`weak_words`](WeakWordsDetectResponseBuilder::weak_words)
    pub fn build(self) -> Result<WeakWordsDetectResponse, BuildError> {
        Ok(WeakWordsDetectResponse {
            weak_words: self
                .weak_words
                .ok_or_else(|| BuildError::missing_field("weak_words"))?,
        })
    }
}
