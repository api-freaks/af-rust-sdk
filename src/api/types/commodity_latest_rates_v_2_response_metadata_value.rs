pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommodityLatestRatesV2ResponseMetadataValue {
    /// Unit of measurement for the commodity (e.g., Bbl, T.oz).
    #[serde(default)]
    pub unit: String,
    /// Quote currency used for this commodity's price.
    #[serde(default)]
    pub quote: String,
}

impl CommodityLatestRatesV2ResponseMetadataValue {
    pub fn builder() -> CommodityLatestRatesV2ResponseMetadataValueBuilder {
        <CommodityLatestRatesV2ResponseMetadataValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommodityLatestRatesV2ResponseMetadataValueBuilder {
    unit: Option<String>,
    quote: Option<String>,
}

impl CommodityLatestRatesV2ResponseMetadataValueBuilder {
    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    pub fn quote(mut self, value: impl Into<String>) -> Self {
        self.quote = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CommodityLatestRatesV2ResponseMetadataValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`unit`](CommodityLatestRatesV2ResponseMetadataValueBuilder::unit)
    /// - [`quote`](CommodityLatestRatesV2ResponseMetadataValueBuilder::quote)
    pub fn build(self) -> Result<CommodityLatestRatesV2ResponseMetadataValue, BuildError> {
        Ok(CommodityLatestRatesV2ResponseMetadataValue {
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
            quote: self
                .quote
                .ok_or_else(|| BuildError::missing_field("quote"))?,
        })
    }
}
