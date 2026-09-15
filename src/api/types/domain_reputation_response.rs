pub use crate::prelude::*;

/// Full domain reputation assessment response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DomainReputationResponse {
    /// Input object containing the analyzed domain.
    #[serde(default)]
    pub input: DomainReputationResponseInput,
    /// Timestamp when the assessment was performed (YYYY-MM-DDTHH:mm:ssZ).
    #[serde(default)]
    pub assessed_at: String,
    /// API / response schema version.
    #[serde(default)]
    pub version: String,
    /// Time taken to process the request, in milliseconds.
    #[serde(default)]
    pub processing_time_ms: i64,
    /// Overall risk assessment for the domain.
    pub risk_category: DomainReputationResponseRiskCategory,
    /// Domain Generation Algorithm (DGA) detection results.
    #[serde(default)]
    pub dga_score: DomainReputationResponseDgaScore,
    /// Trust scoring and supporting signals for the domain.
    #[serde(default)]
    pub trust_signals: DomainReputationResponseTrustSignals,
    /// Assessment of the domain's ability to send and receive email reliably.
    #[serde(default)]
    pub email_deliverability: DomainReputationResponseEmailDeliverability,
    /// Threat intelligence details for the indicator of compromise (IOC).
    pub intelligence: DomainReputationResponseIntelligence,
    /// Summary of reasons behind the risk assessment.
    #[serde(default)]
    pub evidence_summary: DomainReputationResponseEvidenceSummary,
    /// List of errors encountered during processing, if any (e.g. "WHOIS lookup failed"). An empty array means every signal resolved.
    #[serde(default)]
    pub errors: Vec<String>,
}

impl DomainReputationResponse {
    pub fn builder() -> DomainReputationResponseBuilder {
        <DomainReputationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainReputationResponseBuilder {
    input: Option<DomainReputationResponseInput>,
    assessed_at: Option<String>,
    version: Option<String>,
    processing_time_ms: Option<i64>,
    risk_category: Option<DomainReputationResponseRiskCategory>,
    dga_score: Option<DomainReputationResponseDgaScore>,
    trust_signals: Option<DomainReputationResponseTrustSignals>,
    email_deliverability: Option<DomainReputationResponseEmailDeliverability>,
    intelligence: Option<DomainReputationResponseIntelligence>,
    evidence_summary: Option<DomainReputationResponseEvidenceSummary>,
    errors: Option<Vec<String>>,
}

impl DomainReputationResponseBuilder {
    pub fn input(mut self, value: DomainReputationResponseInput) -> Self {
        self.input = Some(value);
        self
    }

    pub fn assessed_at(mut self, value: impl Into<String>) -> Self {
        self.assessed_at = Some(value.into());
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    pub fn processing_time_ms(mut self, value: i64) -> Self {
        self.processing_time_ms = Some(value);
        self
    }

    pub fn risk_category(mut self, value: DomainReputationResponseRiskCategory) -> Self {
        self.risk_category = Some(value);
        self
    }

    pub fn dga_score(mut self, value: DomainReputationResponseDgaScore) -> Self {
        self.dga_score = Some(value);
        self
    }

    pub fn trust_signals(mut self, value: DomainReputationResponseTrustSignals) -> Self {
        self.trust_signals = Some(value);
        self
    }

    pub fn email_deliverability(
        mut self,
        value: DomainReputationResponseEmailDeliverability,
    ) -> Self {
        self.email_deliverability = Some(value);
        self
    }

    pub fn intelligence(mut self, value: DomainReputationResponseIntelligence) -> Self {
        self.intelligence = Some(value);
        self
    }

    pub fn evidence_summary(mut self, value: DomainReputationResponseEvidenceSummary) -> Self {
        self.evidence_summary = Some(value);
        self
    }

    pub fn errors(mut self, value: Vec<String>) -> Self {
        self.errors = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainReputationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`input`](DomainReputationResponseBuilder::input)
    /// - [`assessed_at`](DomainReputationResponseBuilder::assessed_at)
    /// - [`version`](DomainReputationResponseBuilder::version)
    /// - [`processing_time_ms`](DomainReputationResponseBuilder::processing_time_ms)
    /// - [`risk_category`](DomainReputationResponseBuilder::risk_category)
    /// - [`dga_score`](DomainReputationResponseBuilder::dga_score)
    /// - [`trust_signals`](DomainReputationResponseBuilder::trust_signals)
    /// - [`email_deliverability`](DomainReputationResponseBuilder::email_deliverability)
    /// - [`intelligence`](DomainReputationResponseBuilder::intelligence)
    /// - [`evidence_summary`](DomainReputationResponseBuilder::evidence_summary)
    /// - [`errors`](DomainReputationResponseBuilder::errors)
    pub fn build(self) -> Result<DomainReputationResponse, BuildError> {
        Ok(DomainReputationResponse {
            input: self
                .input
                .ok_or_else(|| BuildError::missing_field("input"))?,
            assessed_at: self
                .assessed_at
                .ok_or_else(|| BuildError::missing_field("assessed_at"))?,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
            processing_time_ms: self
                .processing_time_ms
                .ok_or_else(|| BuildError::missing_field("processing_time_ms"))?,
            risk_category: self
                .risk_category
                .ok_or_else(|| BuildError::missing_field("risk_category"))?,
            dga_score: self
                .dga_score
                .ok_or_else(|| BuildError::missing_field("dga_score"))?,
            trust_signals: self
                .trust_signals
                .ok_or_else(|| BuildError::missing_field("trust_signals"))?,
            email_deliverability: self
                .email_deliverability
                .ok_or_else(|| BuildError::missing_field("email_deliverability"))?,
            intelligence: self
                .intelligence
                .ok_or_else(|| BuildError::missing_field("intelligence"))?,
            evidence_summary: self
                .evidence_summary
                .ok_or_else(|| BuildError::missing_field("evidence_summary"))?,
            errors: self
                .errors
                .ok_or_else(|| BuildError::missing_field("errors"))?,
        })
    }
}
