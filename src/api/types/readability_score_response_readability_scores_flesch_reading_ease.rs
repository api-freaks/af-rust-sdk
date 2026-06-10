pub use crate::prelude::*;

/// Flesch Reading Ease
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReadabilityScoreResponseReadabilityScoresFleschReadingEase {
    /// The metric score
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub score: Option<f64>,
    /// Human-readable difficulty level for that metric
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
}

impl ReadabilityScoreResponseReadabilityScoresFleschReadingEase {
    pub fn builder() -> ReadabilityScoreResponseReadabilityScoresFleschReadingEaseBuilder {
        <ReadabilityScoreResponseReadabilityScoresFleschReadingEaseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReadabilityScoreResponseReadabilityScoresFleschReadingEaseBuilder {
    score: Option<f64>,
    level: Option<String>,
}

impl ReadabilityScoreResponseReadabilityScoresFleschReadingEaseBuilder {
    pub fn score(mut self, value: f64) -> Self {
        self.score = Some(value);
        self
    }

    pub fn level(mut self, value: impl Into<String>) -> Self {
        self.level = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReadabilityScoreResponseReadabilityScoresFleschReadingEase`].
    pub fn build(
        self,
    ) -> Result<ReadabilityScoreResponseReadabilityScoresFleschReadingEase, BuildError> {
        Ok(ReadabilityScoreResponseReadabilityScoresFleschReadingEase {
            score: self.score,
            level: self.level,
        })
    }
}
