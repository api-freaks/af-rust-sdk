pub use crate::prelude::*;

/// SMOG Index
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReadabilityScoreResponseReadabilityScoresSmogIndex {
    /// The metric score
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub score: Option<f64>,
    /// Human-readable difficulty level for that metric
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
}

impl ReadabilityScoreResponseReadabilityScoresSmogIndex {
    pub fn builder() -> ReadabilityScoreResponseReadabilityScoresSmogIndexBuilder {
        <ReadabilityScoreResponseReadabilityScoresSmogIndexBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReadabilityScoreResponseReadabilityScoresSmogIndexBuilder {
    score: Option<f64>,
    level: Option<String>,
}

impl ReadabilityScoreResponseReadabilityScoresSmogIndexBuilder {
    pub fn score(mut self, value: f64) -> Self {
        self.score = Some(value);
        self
    }

    pub fn level(mut self, value: impl Into<String>) -> Self {
        self.level = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReadabilityScoreResponseReadabilityScoresSmogIndex`].
    pub fn build(self) -> Result<ReadabilityScoreResponseReadabilityScoresSmogIndex, BuildError> {
        Ok(ReadabilityScoreResponseReadabilityScoresSmogIndex {
            score: self.score,
            level: self.level,
        })
    }
}
