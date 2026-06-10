pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReadabilityScoreResponseSentenceReadabilityItem {
    /// The sentence text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentence: Option<String>,
    /// Readability grade for the sentence
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub readability_grade: Option<f64>,
    /// Difficulty level for the sentence based on the target audience
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readability_level: Option<ReadabilityScoreResponseSentenceReadabilityItemReadabilityLevel>,
}

impl ReadabilityScoreResponseSentenceReadabilityItem {
    pub fn builder() -> ReadabilityScoreResponseSentenceReadabilityItemBuilder {
        <ReadabilityScoreResponseSentenceReadabilityItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReadabilityScoreResponseSentenceReadabilityItemBuilder {
    sentence: Option<String>,
    readability_grade: Option<f64>,
    readability_level: Option<ReadabilityScoreResponseSentenceReadabilityItemReadabilityLevel>,
}

impl ReadabilityScoreResponseSentenceReadabilityItemBuilder {
    pub fn sentence(mut self, value: impl Into<String>) -> Self {
        self.sentence = Some(value.into());
        self
    }

    pub fn readability_grade(mut self, value: f64) -> Self {
        self.readability_grade = Some(value);
        self
    }

    pub fn readability_level(
        mut self,
        value: ReadabilityScoreResponseSentenceReadabilityItemReadabilityLevel,
    ) -> Self {
        self.readability_level = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReadabilityScoreResponseSentenceReadabilityItem`].
    pub fn build(self) -> Result<ReadabilityScoreResponseSentenceReadabilityItem, BuildError> {
        Ok(ReadabilityScoreResponseSentenceReadabilityItem {
            sentence: self.sentence,
            readability_grade: self.readability_grade,
            readability_level: self.readability_level,
        })
    }
}
