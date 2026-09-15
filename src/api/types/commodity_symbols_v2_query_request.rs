pub use crate::prelude::*;

/// Query parameters for commodity_symbols_v2
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommoditySymbolsV2QueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Response format. Currently only `json` is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<CommoditySymbolsV2RequestFormat>,
}

impl CommoditySymbolsV2QueryRequest {
    pub fn builder() -> CommoditySymbolsV2QueryRequestBuilder {
        <CommoditySymbolsV2QueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommoditySymbolsV2QueryRequestBuilder {
    api_key: Option<String>,
    format: Option<CommoditySymbolsV2RequestFormat>,
}

impl CommoditySymbolsV2QueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: CommoditySymbolsV2RequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommoditySymbolsV2QueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](CommoditySymbolsV2QueryRequestBuilder::api_key)
    pub fn build(self) -> Result<CommoditySymbolsV2QueryRequest, BuildError> {
        Ok(CommoditySymbolsV2QueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
        })
    }
}
