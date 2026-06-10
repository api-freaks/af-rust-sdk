pub use crate::prelude::*;

/// Query parameters for commodity_symbols
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommoditySymbolsQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<CommoditySymbolsRequestFormat>,
}

impl CommoditySymbolsQueryRequest {
    pub fn builder() -> CommoditySymbolsQueryRequestBuilder {
        <CommoditySymbolsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommoditySymbolsQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<CommoditySymbolsRequestFormat>,
}

impl CommoditySymbolsQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: CommoditySymbolsRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommoditySymbolsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](CommoditySymbolsQueryRequestBuilder::api_key)
    pub fn build(self) -> Result<CommoditySymbolsQueryRequest, BuildError> {
        Ok(CommoditySymbolsQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
        })
    }
}
