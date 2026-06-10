pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommoditySymbolsResponseSymbolsItemCurrency {
    /// The ISO 4217 currency code (e.g., "USD").
    #[serde(default)]
    pub code: String,
    /// The full name of the currency (e.g., "US Dollar").
    #[serde(default)]
    pub name: String,
    /// The symbol of the currency (e.g., "$").
    #[serde(default)]
    pub symbol: String,
}

impl CommoditySymbolsResponseSymbolsItemCurrency {
    pub fn builder() -> CommoditySymbolsResponseSymbolsItemCurrencyBuilder {
        <CommoditySymbolsResponseSymbolsItemCurrencyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommoditySymbolsResponseSymbolsItemCurrencyBuilder {
    code: Option<String>,
    name: Option<String>,
    symbol: Option<String>,
}

impl CommoditySymbolsResponseSymbolsItemCurrencyBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn symbol(mut self, value: impl Into<String>) -> Self {
        self.symbol = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CommoditySymbolsResponseSymbolsItemCurrency`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](CommoditySymbolsResponseSymbolsItemCurrencyBuilder::code)
    /// - [`name`](CommoditySymbolsResponseSymbolsItemCurrencyBuilder::name)
    /// - [`symbol`](CommoditySymbolsResponseSymbolsItemCurrencyBuilder::symbol)
    pub fn build(self) -> Result<CommoditySymbolsResponseSymbolsItemCurrency, BuildError> {
        Ok(CommoditySymbolsResponseSymbolsItemCurrency {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            symbol: self
                .symbol
                .ok_or_else(|| BuildError::missing_field("symbol"))?,
        })
    }
}
