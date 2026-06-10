pub use crate::prelude::*;

/// Standard readability formula scores, keyed by metric name. Metrics that cannot be computed for the supplied text are omitted.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReadabilityScoreResponseReadabilityScores {
    /// Flesch Reading Ease
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flesch_reading_ease: Option<ReadabilityScoreResponseReadabilityScoresFleschReadingEase>,
    /// Flesch-Kincaid Grade Level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flesch_kincaid_grade: Option<ReadabilityScoreResponseReadabilityScoresFleschKincaidGrade>,
    /// Gunning Fog Index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gunning_fog: Option<ReadabilityScoreResponseReadabilityScoresGunningFog>,
    /// SMOG Index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smog_index: Option<ReadabilityScoreResponseReadabilityScoresSmogIndex>,
    /// Coleman-Liau Index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coleman_liau_index: Option<ReadabilityScoreResponseReadabilityScoresColemanLiauIndex>,
    /// Automated Readability Index (ARI)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_readability_index:
        Option<ReadabilityScoreResponseReadabilityScoresAutomatedReadabilityIndex>,
}

impl ReadabilityScoreResponseReadabilityScores {
    pub fn builder() -> ReadabilityScoreResponseReadabilityScoresBuilder {
        <ReadabilityScoreResponseReadabilityScoresBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReadabilityScoreResponseReadabilityScoresBuilder {
    flesch_reading_ease: Option<ReadabilityScoreResponseReadabilityScoresFleschReadingEase>,
    flesch_kincaid_grade: Option<ReadabilityScoreResponseReadabilityScoresFleschKincaidGrade>,
    gunning_fog: Option<ReadabilityScoreResponseReadabilityScoresGunningFog>,
    smog_index: Option<ReadabilityScoreResponseReadabilityScoresSmogIndex>,
    coleman_liau_index: Option<ReadabilityScoreResponseReadabilityScoresColemanLiauIndex>,
    automated_readability_index:
        Option<ReadabilityScoreResponseReadabilityScoresAutomatedReadabilityIndex>,
}

impl ReadabilityScoreResponseReadabilityScoresBuilder {
    pub fn flesch_reading_ease(
        mut self,
        value: ReadabilityScoreResponseReadabilityScoresFleschReadingEase,
    ) -> Self {
        self.flesch_reading_ease = Some(value);
        self
    }

    pub fn flesch_kincaid_grade(
        mut self,
        value: ReadabilityScoreResponseReadabilityScoresFleschKincaidGrade,
    ) -> Self {
        self.flesch_kincaid_grade = Some(value);
        self
    }

    pub fn gunning_fog(
        mut self,
        value: ReadabilityScoreResponseReadabilityScoresGunningFog,
    ) -> Self {
        self.gunning_fog = Some(value);
        self
    }

    pub fn smog_index(mut self, value: ReadabilityScoreResponseReadabilityScoresSmogIndex) -> Self {
        self.smog_index = Some(value);
        self
    }

    pub fn coleman_liau_index(
        mut self,
        value: ReadabilityScoreResponseReadabilityScoresColemanLiauIndex,
    ) -> Self {
        self.coleman_liau_index = Some(value);
        self
    }

    pub fn automated_readability_index(
        mut self,
        value: ReadabilityScoreResponseReadabilityScoresAutomatedReadabilityIndex,
    ) -> Self {
        self.automated_readability_index = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReadabilityScoreResponseReadabilityScores`].
    pub fn build(self) -> Result<ReadabilityScoreResponseReadabilityScores, BuildError> {
        Ok(ReadabilityScoreResponseReadabilityScores {
            flesch_reading_ease: self.flesch_reading_ease,
            flesch_kincaid_grade: self.flesch_kincaid_grade,
            gunning_fog: self.gunning_fog,
            smog_index: self.smog_index,
            coleman_liau_index: self.coleman_liau_index,
            automated_readability_index: self.automated_readability_index,
        })
    }
}
