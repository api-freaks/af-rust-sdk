pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReadabilityScoreResponse {
    /// Standard readability formula scores, keyed by metric name. Metrics that cannot be computed for the supplied text are omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readability_scores: Option<ReadabilityScoreResponseReadabilityScores>,
    /// Per-sentence readability breakdown
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentence_readability: Option<Vec<ReadabilityScoreResponseSentenceReadabilityItem>>,
    /// Overall readability grade, calculated as the average of sentence readability scores
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub readability_grade: Option<f64>,
}

impl ReadabilityScoreResponse {
    pub fn builder() -> ReadabilityScoreResponseBuilder {
        <ReadabilityScoreResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReadabilityScoreResponseBuilder {
    readability_scores: Option<ReadabilityScoreResponseReadabilityScores>,
    sentence_readability: Option<Vec<ReadabilityScoreResponseSentenceReadabilityItem>>,
    readability_grade: Option<f64>,
}

impl ReadabilityScoreResponseBuilder {
    pub fn readability_scores(mut self, value: ReadabilityScoreResponseReadabilityScores) -> Self {
        self.readability_scores = Some(value);
        self
    }

    pub fn sentence_readability(
        mut self,
        value: Vec<ReadabilityScoreResponseSentenceReadabilityItem>,
    ) -> Self {
        self.sentence_readability = Some(value);
        self
    }

    pub fn readability_grade(mut self, value: f64) -> Self {
        self.readability_grade = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReadabilityScoreResponse`].
    pub fn build(self) -> Result<ReadabilityScoreResponse, BuildError> {
        Ok(ReadabilityScoreResponse {
            readability_scores: self.readability_scores,
            sentence_readability: self.sentence_readability,
            readability_grade: self.readability_grade,
        })
    }
}
