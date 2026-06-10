pub use crate::prelude::*;

/// Coleman-Liau Index
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReadabilityScoreResponseReadabilityScoresColemanLiauIndex {
    /// The metric score
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub score: Option<f64>,
    /// Human-readable difficulty level for that metric
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
}

impl ReadabilityScoreResponseReadabilityScoresColemanLiauIndex {
    pub fn builder() -> ReadabilityScoreResponseReadabilityScoresColemanLiauIndexBuilder {
        <ReadabilityScoreResponseReadabilityScoresColemanLiauIndexBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReadabilityScoreResponseReadabilityScoresColemanLiauIndexBuilder {
    score: Option<f64>,
    level: Option<String>,
}

impl ReadabilityScoreResponseReadabilityScoresColemanLiauIndexBuilder {
    pub fn score(mut self, value: f64) -> Self {
        self.score = Some(value);
        self
    }

    pub fn level(mut self, value: impl Into<String>) -> Self {
        self.level = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReadabilityScoreResponseReadabilityScoresColemanLiauIndex`].
    pub fn build(
        self,
    ) -> Result<ReadabilityScoreResponseReadabilityScoresColemanLiauIndex, BuildError> {
        Ok(ReadabilityScoreResponseReadabilityScoresColemanLiauIndex {
            score: self.score,
            level: self.level,
        })
    }
}
