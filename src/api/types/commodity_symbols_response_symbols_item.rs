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
    /// The rate at which this commodity's price is updated. New values may be added over time.
    #[serde(rename = "updateInterval")]
    pub update_interval: CommoditySymbolsResponseSymbolsItemUpdateInterval,
    #[serde(default)]
    pub currency: CommoditySymbolsResponseSymbolsItemCurrency,
    #[serde(default)]
    pub unit: CommoditySymbolsResponseSymbolsItemUnit,
    /// Short description of the commodity. May be an empty string for some symbols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Data source for the symbol (e.g., World Bank). Present only for some symbols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    /// Present only when status is "inactive". Date the symbol was deprecated (YYYY-MM-DD).
    #[serde(rename = "deprecationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_date: Option<NaiveDate>,
    /// Optional. Present only for dated futures contracts (for example `WTIOIL-FUT-DEC26`). Contract type, currently always `futures`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,
    /// Optional. Present only for dated futures contracts. Delivery month of the contract (YYYY-MM), for example `2026-12`.
    #[serde(rename = "deliveryMonth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_month: Option<String>,
    /// Optional. Present for most dated futures contracts. Last trading date of the contract (YYYY-MM-DD), for example `2026-11-20`.
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<NaiveDate>,
    /// Optional. Present only for dated futures contracts. Symbol of the continuous futures series this contract belongs to (for example `WTIOIL-FUT`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,
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
    description: Option<String>,
    exchange: Option<String>,
    deprecation_date: Option<NaiveDate>,
    contract: Option<String>,
    delivery_month: Option<String>,
    expires_at: Option<NaiveDate>,
    underlying: Option<String>,
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

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
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

    pub fn contract(mut self, value: impl Into<String>) -> Self {
        self.contract = Some(value.into());
        self
    }

    pub fn delivery_month(mut self, value: impl Into<String>) -> Self {
        self.delivery_month = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: NaiveDate) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn underlying(mut self, value: impl Into<String>) -> Self {
        self.underlying = Some(value.into());
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
            description: self.description,
            exchange: self.exchange,
            deprecation_date: self.deprecation_date,
            contract: self.contract,
            delivery_month: self.delivery_month,
            expires_at: self.expires_at,
            underlying: self.underlying,
        })
    }
}
