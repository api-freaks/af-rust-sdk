pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CommoditySymbolsResponseSymbolsItem {
    /// The unique ticker symbol for the commodity (e.g., "XAU", "NG-FUT").
    #[serde(default)]
    pub symbol: String,
    /// The full name of the commodity (e.g., "Gold", "Natural Gas Futures").
    #[serde(default)]
    pub name: String,
    /// The category the commodity belongs to (e.g., "Metals", "Energy").
    #[serde(default)]
    pub category: String,
    /// The current status of the commodity. Possible value: "active".
    #[serde(default)]
    pub status: String,
    /// The rate at which this commodity's price is updated.
    #[serde(rename = "updateInterval")]
    pub update_interval: CommoditySymbolsResponseSymbolsItemUpdateInterval,
    #[serde(default)]
    pub currency: CommoditySymbolsResponseSymbolsItemCurrency,
    #[serde(default)]
    pub unit: CommoditySymbolsResponseSymbolsItemUnit,
}

impl CommoditySymbolsResponseSymbolsItem {
    pub fn builder() -> CommoditySymbolsResponseSymbolsItemBuilder {
        <CommoditySymbolsResponseSymbolsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommoditySymbolsResponseSymbolsItemBuilder {
    symbol: Option<String>,
    name: Option<String>,
    category: Option<String>,
    status: Option<String>,
    update_interval: Option<CommoditySymbolsResponseSymbolsItemUpdateInterval>,
    currency: Option<CommoditySymbolsResponseSymbolsItemCurrency>,
    unit: Option<CommoditySymbolsResponseSymbolsItemUnit>,
}

impl CommoditySymbolsResponseSymbolsItemBuilder {
    pub fn symbol(mut self, value: impl Into<String>) -> Self {
        self.symbol = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn update_interval(
        mut self,
        value: CommoditySymbolsResponseSymbolsItemUpdateInterval,
    ) -> Self {
        self.update_interval = Some(value);
        self
    }

    pub fn currency(mut self, value: CommoditySymbolsResponseSymbolsItemCurrency) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn unit(mut self, value: CommoditySymbolsResponseSymbolsItemUnit) -> Self {
        self.unit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommoditySymbolsResponseSymbolsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`symbol`](CommoditySymbolsResponseSymbolsItemBuilder::symbol)
    /// - [`name`](CommoditySymbolsResponseSymbolsItemBuilder::name)
    /// - [`category`](CommoditySymbolsResponseSymbolsItemBuilder::category)
    /// - [`status`](CommoditySymbolsResponseSymbolsItemBuilder::status)
    /// - [`update_interval`](CommoditySymbolsResponseSymbolsItemBuilder::update_interval)
    /// - [`currency`](CommoditySymbolsResponseSymbolsItemBuilder::currency)
    /// - [`unit`](CommoditySymbolsResponseSymbolsItemBuilder::unit)
    pub fn build(self) -> Result<CommoditySymbolsResponseSymbolsItem, BuildError> {
        Ok(CommoditySymbolsResponseSymbolsItem {
            symbol: self
                .symbol
                .ok_or_else(|| BuildError::missing_field("symbol"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            update_interval: self
                .update_interval
                .ok_or_else(|| BuildError::missing_field("update_interval"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
        })
    }
}
