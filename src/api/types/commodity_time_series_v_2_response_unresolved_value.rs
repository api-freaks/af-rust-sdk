pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommodityTimeSeriesV2ResponseUnresolvedValue {
    /// Detail message explaining why this symbol was not resolved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Optional list of supported symbols that closely match the unresolved symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestions: Option<Vec<String>>,
}

impl CommodityTimeSeriesV2ResponseUnresolvedValue {
    pub fn builder() -> CommodityTimeSeriesV2ResponseUnresolvedValueBuilder {
        <CommodityTimeSeriesV2ResponseUnresolvedValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityTimeSeriesV2ResponseUnresolvedValueBuilder {
    message: Option<String>,
    suggestions: Option<Vec<String>>,
}

impl CommodityTimeSeriesV2ResponseUnresolvedValueBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn suggestions(mut self, value: Vec<String>) -> Self {
        self.suggestions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommodityTimeSeriesV2ResponseUnresolvedValue`].
    pub fn build(self) -> Result<CommodityTimeSeriesV2ResponseUnresolvedValue, BuildError> {
        Ok(CommodityTimeSeriesV2ResponseUnresolvedValue {
            message: self.message,
            suggestions: self.suggestions,
        })
    }
}
