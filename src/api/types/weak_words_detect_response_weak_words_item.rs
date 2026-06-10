pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WeakWordsDetectResponseWeakWordsItem {
    /// The detected weak word
    #[serde(default)]
    pub word: String,
    /// Zero-based word position in the input text
    #[serde(default)]
    pub offset: i64,
}

impl WeakWordsDetectResponseWeakWordsItem {
    pub fn builder() -> WeakWordsDetectResponseWeakWordsItemBuilder {
        <WeakWordsDetectResponseWeakWordsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WeakWordsDetectResponseWeakWordsItemBuilder {
    word: Option<String>,
    offset: Option<i64>,
}

impl WeakWordsDetectResponseWeakWordsItemBuilder {
    pub fn word(mut self, value: impl Into<String>) -> Self {
        self.word = Some(value.into());
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WeakWordsDetectResponseWeakWordsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`word`](WeakWordsDetectResponseWeakWordsItemBuilder::word)
    /// - [`offset`](WeakWordsDetectResponseWeakWordsItemBuilder::offset)
    pub fn build(self) -> Result<WeakWordsDetectResponseWeakWordsItem, BuildError> {
        Ok(WeakWordsDetectResponseWeakWordsItem {
            word: self.word.ok_or_else(|| BuildError::missing_field("word"))?,
            offset: self
                .offset
                .ok_or_else(|| BuildError::missing_field("offset"))?,
        })
    }
}
