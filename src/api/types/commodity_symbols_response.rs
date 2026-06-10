pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommoditySymbolsResponse {
    /// Indicates whether the API request was successful.
    #[serde(default)]
    pub success: bool,
    /// An array of commodity symbol objects.
    #[serde(default)]
    pub symbols: Vec<CommoditySymbolsResponseSymbolsItem>,
}

impl CommoditySymbolsResponse {
    pub fn builder() -> CommoditySymbolsResponseBuilder {
        <CommoditySymbolsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommoditySymbolsResponseBuilder {
    success: Option<bool>,
    symbols: Option<Vec<CommoditySymbolsResponseSymbolsItem>>,
}

impl CommoditySymbolsResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn symbols(mut self, value: Vec<CommoditySymbolsResponseSymbolsItem>) -> Self {
        self.symbols = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommoditySymbolsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](CommoditySymbolsResponseBuilder::success)
    /// - [`symbols`](CommoditySymbolsResponseBuilder::symbols)
    pub fn build(self) -> Result<CommoditySymbolsResponse, BuildError> {
        Ok(CommoditySymbolsResponse {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            symbols: self
                .symbols
                .ok_or_else(|| BuildError::missing_field("symbols"))?,
        })
    }
}
