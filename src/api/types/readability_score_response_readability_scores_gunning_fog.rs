pub use crate::prelude::*;

/// Gunning Fog Index
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReadabilityScoreResponseReadabilityScoresGunningFog {
    /// The metric score
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub score: Option<f64>,
    /// Human-readable difficulty level for that metric
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
}

impl ReadabilityScoreResponseReadabilityScoresGunningFog {
    pub fn builder() -> ReadabilityScoreResponseReadabilityScoresGunningFogBuilder {
        <ReadabilityScoreResponseReadabilityScoresGunningFogBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReadabilityScoreResponseReadabilityScoresGunningFogBuilder {
    score: Option<f64>,
    level: Option<String>,
}

impl ReadabilityScoreResponseReadabilityScoresGunningFogBuilder {
    pub fn score(mut self, value: f64) -> Self {
        self.score = Some(value);
        self
    }

    pub fn level(mut self, value: impl Into<String>) -> Self {
        self.level = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReadabilityScoreResponseReadabilityScoresGunningFog`].
    pub fn build(self) -> Result<ReadabilityScoreResponseReadabilityScoresGunningFog, BuildError> {
        Ok(ReadabilityScoreResponseReadabilityScoresGunningFog {
            score: self.score,
            level: self.level,
        })
    }
}
