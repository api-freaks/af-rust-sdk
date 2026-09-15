pub use crate::prelude::*;

/// Underlying lexical / statistical features used in DGA detection.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DomainReputationResponseDgaScoreFeatures {
    /// Length of the domain name.
    #[serde(default)]
    pub domain_length: i64,
    /// Ratio of vowels to consonants in the domain.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub vowel_consonant_ratio: f64,
    /// N-gram perplexity score of the domain string.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub ngram_perplexity: f64,
    /// Shannon entropy of the domain string.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub shannon_entropy: f64,
    /// Ratio of digits to letters in the domain.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub digit_letter_ratio: f64,
    /// Maximum consecutive consonant streak in the domain.
    #[serde(default)]
    pub consonant_streak_max: i64,
    /// Indicates if the TLD belongs to a known DGA set.
    #[serde(default)]
    pub tld_in_known_dga_set: bool,
}

impl DomainReputationResponseDgaScoreFeatures {
    pub fn builder() -> DomainReputationResponseDgaScoreFeaturesBuilder {
        <DomainReputationResponseDgaScoreFeaturesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainReputationResponseDgaScoreFeaturesBuilder {
    domain_length: Option<i64>,
    vowel_consonant_ratio: Option<f64>,
    ngram_perplexity: Option<f64>,
    shannon_entropy: Option<f64>,
    digit_letter_ratio: Option<f64>,
    consonant_streak_max: Option<i64>,
    tld_in_known_dga_set: Option<bool>,
}

impl DomainReputationResponseDgaScoreFeaturesBuilder {
    pub fn domain_length(mut self, value: i64) -> Self {
        self.domain_length = Some(value);
        self
    }

    pub fn vowel_consonant_ratio(mut self, value: f64) -> Self {
        self.vowel_consonant_ratio = Some(value);
        self
    }

    pub fn ngram_perplexity(mut self, value: f64) -> Self {
        self.ngram_perplexity = Some(value);
        self
    }

    pub fn shannon_entropy(mut self, value: f64) -> Self {
        self.shannon_entropy = Some(value);
        self
    }

    pub fn digit_letter_ratio(mut self, value: f64) -> Self {
        self.digit_letter_ratio = Some(value);
        self
    }

    pub fn consonant_streak_max(mut self, value: i64) -> Self {
        self.consonant_streak_max = Some(value);
        self
    }

    pub fn tld_in_known_dga_set(mut self, value: bool) -> Self {
        self.tld_in_known_dga_set = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainReputationResponseDgaScoreFeatures`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain_length`](DomainReputationResponseDgaScoreFeaturesBuilder::domain_length)
    /// - [`vowel_consonant_ratio`](DomainReputationResponseDgaScoreFeaturesBuilder::vowel_consonant_ratio)
    /// - [`ngram_perplexity`](DomainReputationResponseDgaScoreFeaturesBuilder::ngram_perplexity)
    /// - [`shannon_entropy`](DomainReputationResponseDgaScoreFeaturesBuilder::shannon_entropy)
    /// - [`digit_letter_ratio`](DomainReputationResponseDgaScoreFeaturesBuilder::digit_letter_ratio)
    /// - [`consonant_streak_max`](DomainReputationResponseDgaScoreFeaturesBuilder::consonant_streak_max)
    /// - [`tld_in_known_dga_set`](DomainReputationResponseDgaScoreFeaturesBuilder::tld_in_known_dga_set)
    pub fn build(self) -> Result<DomainReputationResponseDgaScoreFeatures, BuildError> {
        Ok(DomainReputationResponseDgaScoreFeatures {
            domain_length: self
                .domain_length
                .ok_or_else(|| BuildError::missing_field("domain_length"))?,
            vowel_consonant_ratio: self
                .vowel_consonant_ratio
                .ok_or_else(|| BuildError::missing_field("vowel_consonant_ratio"))?,
            ngram_perplexity: self
                .ngram_perplexity
                .ok_or_else(|| BuildError::missing_field("ngram_perplexity"))?,
            shannon_entropy: self
                .shannon_entropy
                .ok_or_else(|| BuildError::missing_field("shannon_entropy"))?,
            digit_letter_ratio: self
                .digit_letter_ratio
                .ok_or_else(|| BuildError::missing_field("digit_letter_ratio"))?,
            consonant_streak_max: self
                .consonant_streak_max
                .ok_or_else(|| BuildError::missing_field("consonant_streak_max"))?,
            tld_in_known_dga_set: self
                .tld_in_known_dga_set
                .ok_or_else(|| BuildError::missing_field("tld_in_known_dga_set"))?,
        })
    }
}
