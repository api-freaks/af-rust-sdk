pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BulkCurrentWeatherResponseBulkItem {
    /// Location information
    pub location: BulkCurrentWeatherResponseBulkItemLocation,
    /// Current weather data
    #[serde(default)]
    pub current: BulkCurrentWeatherResponseBulkItemCurrent,
}

impl BulkCurrentWeatherResponseBulkItem {
    pub fn builder() -> BulkCurrentWeatherResponseBulkItemBuilder {
        <BulkCurrentWeatherResponseBulkItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkCurrentWeatherResponseBulkItemBuilder {
    location: Option<BulkCurrentWeatherResponseBulkItemLocation>,
    current: Option<BulkCurrentWeatherResponseBulkItemCurrent>,
}

impl BulkCurrentWeatherResponseBulkItemBuilder {
    pub fn location(mut self, value: BulkCurrentWeatherResponseBulkItemLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn current(mut self, value: BulkCurrentWeatherResponseBulkItemCurrent) -> Self {
        self.current = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkCurrentWeatherResponseBulkItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`location`](BulkCurrentWeatherResponseBulkItemBuilder::location)
    /// - [`current`](BulkCurrentWeatherResponseBulkItemBuilder::current)
    pub fn build(self) -> Result<BulkCurrentWeatherResponseBulkItem, BuildError> {
        Ok(BulkCurrentWeatherResponseBulkItem {
            location: self
                .location
                .ok_or_else(|| BuildError::missing_field("location"))?,
            current: self
                .current
                .ok_or_else(|| BuildError::missing_field("current"))?,
        })
    }
}
