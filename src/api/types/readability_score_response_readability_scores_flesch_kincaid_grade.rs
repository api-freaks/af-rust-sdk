pub use crate::prelude::*;

/// Flesch-Kincaid Grade Level
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReadabilityScoreResponseReadabilityScoresFleschKincaidGrade {
    /// The metric score
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub score: Option<f64>,
    /// Human-readable difficulty level for that metric
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
}

impl ReadabilityScoreResponseReadabilityScoresFleschKincaidGrade {
    pub fn builder() -> ReadabilityScoreResponseReadabilityScoresFleschKincaidGradeBuilder {
        <ReadabilityScoreResponseReadabilityScoresFleschKincaidGradeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReadabilityScoreResponseReadabilityScoresFleschKincaidGradeBuilder {
    score: Option<f64>,
    level: Option<String>,
}

impl ReadabilityScoreResponseReadabilityScoresFleschKincaidGradeBuilder {
    pub fn score(mut self, value: f64) -> Self {
        self.score = Some(value);
        self
    }

    pub fn level(mut self, value: impl Into<String>) -> Self {
        self.level = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReadabilityScoreResponseReadabilityScoresFleschKincaidGrade`].
    pub fn build(
        self,
    ) -> Result<ReadabilityScoreResponseReadabilityScoresFleschKincaidGrade, BuildError> {
        Ok(
            ReadabilityScoreResponseReadabilityScoresFleschKincaidGrade {
                score: self.score,
                level: self.level,
            },
        )
    }
}
