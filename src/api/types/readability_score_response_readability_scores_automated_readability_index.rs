pub use crate::prelude::*;

/// Automated Readability Index (ARI)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReadabilityScoreResponseReadabilityScoresAutomatedReadabilityIndex {
    /// The metric score
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub score: Option<f64>,
    /// Human-readable difficulty level for that metric
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
}

impl ReadabilityScoreResponseReadabilityScoresAutomatedReadabilityIndex {
    pub fn builder() -> ReadabilityScoreResponseReadabilityScoresAutomatedReadabilityIndexBuilder {
        <ReadabilityScoreResponseReadabilityScoresAutomatedReadabilityIndexBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReadabilityScoreResponseReadabilityScoresAutomatedReadabilityIndexBuilder {
    score: Option<f64>,
    level: Option<String>,
}

impl ReadabilityScoreResponseReadabilityScoresAutomatedReadabilityIndexBuilder {
    pub fn score(mut self, value: f64) -> Self {
        self.score = Some(value);
        self
    }

    pub fn level(mut self, value: impl Into<String>) -> Self {
        self.level = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReadabilityScoreResponseReadabilityScoresAutomatedReadabilityIndex`].
    pub fn build(
        self,
    ) -> Result<ReadabilityScoreResponseReadabilityScoresAutomatedReadabilityIndex, BuildError>
    {
        Ok(
            ReadabilityScoreResponseReadabilityScoresAutomatedReadabilityIndex {
                score: self.score,
                level: self.level,
            },
        )
    }
}
