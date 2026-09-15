pub use crate::prelude::*;

/// A related indicator of compromise.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DomainReputationResponseIntelligenceRelatedIocsItem {
    /// Type of the related IOC (e.g. ipv4, ipv6).
    #[serde(default)]
    pub r#type: String,
    /// Value of the related IOC.
    #[serde(default)]
    pub value: String,
    /// Confidence score for the related IOC (0-1).
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub confidence: f64,
}

impl DomainReputationResponseIntelligenceRelatedIocsItem {
    pub fn builder() -> DomainReputationResponseIntelligenceRelatedIocsItemBuilder {
        <DomainReputationResponseIntelligenceRelatedIocsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainReputationResponseIntelligenceRelatedIocsItemBuilder {
    r#type: Option<String>,
    value: Option<String>,
    confidence: Option<f64>,
}

impl DomainReputationResponseIntelligenceRelatedIocsItemBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn confidence(mut self, value: f64) -> Self {
        self.confidence = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainReputationResponseIntelligenceRelatedIocsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](DomainReputationResponseIntelligenceRelatedIocsItemBuilder::r#type)
    /// - [`value`](DomainReputationResponseIntelligenceRelatedIocsItemBuilder::value)
    /// - [`confidence`](DomainReputationResponseIntelligenceRelatedIocsItemBuilder::confidence)
    pub fn build(self) -> Result<DomainReputationResponseIntelligenceRelatedIocsItem, BuildError> {
        Ok(DomainReputationResponseIntelligenceRelatedIocsItem {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
            confidence: self
                .confidence
                .ok_or_else(|| BuildError::missing_field("confidence"))?,
        })
    }
}
