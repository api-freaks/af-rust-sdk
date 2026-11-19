pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommodityLatestRatesV2ResponseUnresolvedValue {
    /// Detail message explaining why this symbol was not resolved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Optional list of supported symbols that closely match the unresolved symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestions: Option<Vec<String>>,
}

impl CommodityLatestRatesV2ResponseUnresolvedValue {
    pub fn builder() -> CommodityLatestRatesV2ResponseUnresolvedValueBuilder {
        <CommodityLatestRatesV2ResponseUnresolvedValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityLatestRatesV2ResponseUnresolvedValueBuilder {
    message: Option<String>,
    suggestions: Option<Vec<String>>,
}

impl CommodityLatestRatesV2ResponseUnresolvedValueBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn suggestions(mut self, value: Vec<String>) -> Self {
        self.suggestions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommodityLatestRatesV2ResponseUnresolvedValue`].
    pub fn build(self) -> Result<CommodityLatestRatesV2ResponseUnresolvedValue, BuildError> {
        Ok(CommodityLatestRatesV2ResponseUnresolvedValue {
            message: self.message,
            suggestions: self.suggestions,
        })
    }
}
