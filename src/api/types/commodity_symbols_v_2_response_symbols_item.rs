pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CommoditySymbolsV2ResponseSymbolsItem {
    /// Unique identifier/ticker symbol for the commodity (e.g., XAU, NG-FUT). Use this value in the symbols parameter of the rate endpoints.
    #[serde(default)]
    pub symbol: String,
    /// Full name of the commodity (e.g., Gold, Natural Gas Futures).
    #[serde(default)]
    pub name: String,
    /// Short description of the commodity. May be an empty string for some symbols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Category the commodity belongs to (e.g., Metals, Energy, Agriculture, Industrial, Raw Materials, Oils and Meals, Textiles, Meats, Poultry, Livestock).
    #[serde(default)]
    pub category: String,
    /// Current status of the commodity. "inactive" means the symbol is deprecated - latest rates are unavailable, but historical rates remain available up to its deprecationDate.
    pub status: CommoditySymbolsV2ResponseSymbolsItemStatus,
    /// The rate at which this commodity's price is updated.
    #[serde(rename = "updateInterval")]
    pub update_interval: CommoditySymbolsV2ResponseSymbolsItemUpdateInterval,
    /// Data source for the symbol (e.g., World Bank). Present only for some symbols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    /// Present only when status is "inactive". Date the symbol was deprecated (YYYY-MM-DD).
    #[serde(rename = "deprecationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_date: Option<NaiveDate>,
    #[serde(default)]
    pub currency: CommoditySymbolsV2ResponseSymbolsItemCurrency,
    #[serde(default)]
    pub unit: CommoditySymbolsV2ResponseSymbolsItemUnit,
}

impl CommoditySymbolsV2ResponseSymbolsItem {
    pub fn builder() -> CommoditySymbolsV2ResponseSymbolsItemBuilder {
        <CommoditySymbolsV2ResponseSymbolsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommoditySymbolsV2ResponseSymbolsItemBuilder {
    symbol: Option<String>,
    name: Option<String>,
    description: Option<String>,
    category: Option<String>,
    status: Option<CommoditySymbolsV2ResponseSymbolsItemStatus>,
    update_interval: Option<CommoditySymbolsV2ResponseSymbolsItemUpdateInterval>,
    exchange: Option<String>,
    deprecation_date: Option<NaiveDate>,
    currency: Option<CommoditySymbolsV2ResponseSymbolsItemCurrency>,
    unit: Option<CommoditySymbolsV2ResponseSymbolsItemUnit>,
}

impl CommoditySymbolsV2ResponseSymbolsItemBuilder {
    pub fn symbol(mut self, value: impl Into<String>) -> Self {
        self.symbol = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn status(mut self, value: CommoditySymbolsV2ResponseSymbolsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn update_interval(
        mut self,
        value: CommoditySymbolsV2ResponseSymbolsItemUpdateInterval,
    ) -> Self {
        self.update_interval = Some(value);
        self
    }

    pub fn exchange(mut self, value: impl Into<String>) -> Self {
        self.exchange = Some(value.into());
        self
    }

    pub fn deprecation_date(mut self, value: NaiveDate) -> Self {
        self.deprecation_date = Some(value);
        self
    }

    pub fn currency(mut self, value: CommoditySymbolsV2ResponseSymbolsItemCurrency) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn unit(mut self, value: CommoditySymbolsV2ResponseSymbolsItemUnit) -> Self {
        self.unit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommoditySymbolsV2ResponseSymbolsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`symbol`](CommoditySymbolsV2ResponseSymbolsItemBuilder::symbol)
    /// - [`name`](CommoditySymbolsV2ResponseSymbolsItemBuilder::name)
    /// - [`category`](CommoditySymbolsV2ResponseSymbolsItemBuilder::category)
    /// - [`status`](CommoditySymbolsV2ResponseSymbolsItemBuilder::status)
    /// - [`update_interval`](CommoditySymbolsV2ResponseSymbolsItemBuilder::update_interval)
    /// - [`currency`](CommoditySymbolsV2ResponseSymbolsItemBuilder::currency)
    /// - [`unit`](CommoditySymbolsV2ResponseSymbolsItemBuilder::unit)
    pub fn build(self) -> Result<CommoditySymbolsV2ResponseSymbolsItem, BuildError> {
        Ok(CommoditySymbolsV2ResponseSymbolsItem {
            symbol: self
                .symbol
                .ok_or_else(|| BuildError::missing_field("symbol"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            update_interval: self
                .update_interval
                .ok_or_else(|| BuildError::missing_field("update_interval"))?,
            exchange: self.exchange,
            deprecation_date: self.deprecation_date,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
        })
    }
}
