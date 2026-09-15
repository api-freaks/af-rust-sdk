pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommoditySymbolsV2ResponseSymbolsItemUnit {
    /// The abbreviated unit symbol (e.g., "T.oz", "MMBtu").
    #[serde(default)]
    pub symbol: String,
    /// The full name of the unit of measurement.
    #[serde(default)]
    pub name: String,
}

impl CommoditySymbolsV2ResponseSymbolsItemUnit {
    pub fn builder() -> CommoditySymbolsV2ResponseSymbolsItemUnitBuilder {
        <CommoditySymbolsV2ResponseSymbolsItemUnitBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommoditySymbolsV2ResponseSymbolsItemUnitBuilder {
    symbol: Option<String>,
    name: Option<String>,
}

impl CommoditySymbolsV2ResponseSymbolsItemUnitBuilder {
    pub fn symbol(mut self, value: impl Into<String>) -> Self {
        self.symbol = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CommoditySymbolsV2ResponseSymbolsItemUnit`].
    /// This method will fail if any of the following fields are not set:
    /// - [`symbol`](CommoditySymbolsV2ResponseSymbolsItemUnitBuilder::symbol)
    /// - [`name`](CommoditySymbolsV2ResponseSymbolsItemUnitBuilder::name)
    pub fn build(self) -> Result<CommoditySymbolsV2ResponseSymbolsItemUnit, BuildError> {
        Ok(CommoditySymbolsV2ResponseSymbolsItemUnit {
            symbol: self
                .symbol
                .ok_or_else(|| BuildError::missing_field("symbol"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
