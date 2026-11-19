pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommodityIntradayRatesV2ResponseUnresolvedValue {
    /// Detail message explaining how this symbol was handled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The supported symbol the request was answered with.
    #[serde(rename = "resolvedTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_to: Option<String>,
    /// Optional list of supported symbols that closely match the unresolved symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestions: Option<Vec<String>>,
}

impl CommodityIntradayRatesV2ResponseUnresolvedValue {
    pub fn builder() -> CommodityIntradayRatesV2ResponseUnresolvedValueBuilder {
        <CommodityIntradayRatesV2ResponseUnresolvedValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityIntradayRatesV2ResponseUnresolvedValueBuilder {
    message: Option<String>,
    resolved_to: Option<String>,
    suggestions: Option<Vec<String>>,
}

impl CommodityIntradayRatesV2ResponseUnresolvedValueBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn resolved_to(mut self, value: impl Into<String>) -> Self {
        self.resolved_to = Some(value.into());
        self
    }

    pub fn suggestions(mut self, value: Vec<String>) -> Self {
        self.suggestions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommodityIntradayRatesV2ResponseUnresolvedValue`].
    pub fn build(self) -> Result<CommodityIntradayRatesV2ResponseUnresolvedValue, BuildError> {
        Ok(CommodityIntradayRatesV2ResponseUnresolvedValue {
            message: self.message,
            resolved_to: self.resolved_to,
            suggestions: self.suggestions,
        })
    }
}
