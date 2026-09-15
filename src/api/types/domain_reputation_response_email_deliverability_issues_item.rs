pub use crate::prelude::*;

/// A detected email deliverability issue or misconfiguration.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainReputationResponseEmailDeliverabilityIssuesItem {
    /// Machine-readable code identifying the specific deliverability issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Severity level assigned to the detected issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    /// Human-readable explanation of the issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Plain-language description of the exact fix to apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<String>,
}

impl DomainReputationResponseEmailDeliverabilityIssuesItem {
    pub fn builder() -> DomainReputationResponseEmailDeliverabilityIssuesItemBuilder {
        <DomainReputationResponseEmailDeliverabilityIssuesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainReputationResponseEmailDeliverabilityIssuesItemBuilder {
    code: Option<String>,
    severity: Option<String>,
    message: Option<String>,
    recommendation: Option<String>,
}

impl DomainReputationResponseEmailDeliverabilityIssuesItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn severity(mut self, value: impl Into<String>) -> Self {
        self.severity = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn recommendation(mut self, value: impl Into<String>) -> Self {
        self.recommendation = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainReputationResponseEmailDeliverabilityIssuesItem`].
    pub fn build(
        self,
    ) -> Result<DomainReputationResponseEmailDeliverabilityIssuesItem, BuildError> {
        Ok(DomainReputationResponseEmailDeliverabilityIssuesItem {
            code: self.code,
            severity: self.severity,
            message: self.message,
            recommendation: self.recommendation,
        })
    }
}
