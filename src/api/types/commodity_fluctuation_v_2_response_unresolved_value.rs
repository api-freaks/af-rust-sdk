pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommodityFluctuationV2ResponseUnresolvedValue {
    /// Detail message explaining why this symbol was not resolved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Optional list of supported symbols that closely match the unresolved symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestions: Option<Vec<String>>,
}

impl CommodityFluctuationV2ResponseUnresolvedValue {
    pub fn builder() -> CommodityFluctuationV2ResponseUnresolvedValueBuilder {
        <CommodityFluctuationV2ResponseUnresolvedValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityFluctuationV2ResponseUnresolvedValueBuilder {
    message: Option<String>,
    suggestions: Option<Vec<String>>,
}

impl CommodityFluctuationV2ResponseUnresolvedValueBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn suggestions(mut self, value: Vec<String>) -> Self {
        self.suggestions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommodityFluctuationV2ResponseUnresolvedValue`].
    pub fn build(self) -> Result<CommodityFluctuationV2ResponseUnresolvedValue, BuildError> {
        Ok(CommodityFluctuationV2ResponseUnresolvedValue {
            message: self.message,
            suggestions: self.suggestions,
        })
    }
}
