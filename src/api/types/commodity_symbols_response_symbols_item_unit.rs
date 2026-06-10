pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommoditySymbolsResponseSymbolsItemUnit {
    /// The abbreviated unit symbol (e.g., "T.oz", "MMBtu").
    #[serde(default)]
    pub symbol: String,
    /// The full name of the unit of measurement.
    #[serde(default)]
    pub name: String,
}

impl CommoditySymbolsResponseSymbolsItemUnit {
    pub fn builder() -> CommoditySymbolsResponseSymbolsItemUnitBuilder {
        <CommoditySymbolsResponseSymbolsItemUnitBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommoditySymbolsResponseSymbolsItemUnitBuilder {
    symbol: Option<String>,
    name: Option<String>,
}

impl CommoditySymbolsResponseSymbolsItemUnitBuilder {
    pub fn symbol(mut self, value: impl Into<String>) -> Self {
        self.symbol = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CommoditySymbolsResponseSymbolsItemUnit`].
    /// This method will fail if any of the following fields are not set:
    /// - [`symbol`](CommoditySymbolsResponseSymbolsItemUnitBuilder::symbol)
    /// - [`name`](CommoditySymbolsResponseSymbolsItemUnitBuilder::name)
    pub fn build(self) -> Result<CommoditySymbolsResponseSymbolsItemUnit, BuildError> {
        Ok(CommoditySymbolsResponseSymbolsItemUnit {
            symbol: self
                .symbol
                .ok_or_else(|| BuildError::missing_field("symbol"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
