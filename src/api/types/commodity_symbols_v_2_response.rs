pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommoditySymbolsV2Response {
    /// API request success indicator. "true" for successful requests.
    #[serde(default)]
    pub success: bool,
    /// An array of commodity symbol objects.
    #[serde(default)]
    pub symbols: Vec<CommoditySymbolsV2ResponseSymbolsItem>,
}

impl CommoditySymbolsV2Response {
    pub fn builder() -> CommoditySymbolsV2ResponseBuilder {
        <CommoditySymbolsV2ResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommoditySymbolsV2ResponseBuilder {
    success: Option<bool>,
    symbols: Option<Vec<CommoditySymbolsV2ResponseSymbolsItem>>,
}

impl CommoditySymbolsV2ResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn symbols(mut self, value: Vec<CommoditySymbolsV2ResponseSymbolsItem>) -> Self {
        self.symbols = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommoditySymbolsV2Response`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](CommoditySymbolsV2ResponseBuilder::success)
    /// - [`symbols`](CommoditySymbolsV2ResponseBuilder::symbols)
    pub fn build(self) -> Result<CommoditySymbolsV2Response, BuildError> {
        Ok(CommoditySymbolsV2Response {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            symbols: self
                .symbols
                .ok_or_else(|| BuildError::missing_field("symbols"))?,
        })
    }
}
