pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkCurrentWeatherResponse {
    /// Array of weather data
    #[serde(default)]
    pub bulk: Vec<BulkCurrentWeatherResponseBulkItem>,
}

impl BulkCurrentWeatherResponse {
    pub fn builder() -> BulkCurrentWeatherResponseBuilder {
        <BulkCurrentWeatherResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkCurrentWeatherResponseBuilder {
    bulk: Option<Vec<BulkCurrentWeatherResponseBulkItem>>,
}

impl BulkCurrentWeatherResponseBuilder {
    pub fn bulk(mut self, value: Vec<BulkCurrentWeatherResponseBulkItem>) -> Self {
        self.bulk = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkCurrentWeatherResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bulk`](BulkCurrentWeatherResponseBuilder::bulk)
    pub fn build(self) -> Result<BulkCurrentWeatherResponse, BuildError> {
        Ok(BulkCurrentWeatherResponse {
            bulk: self.bulk.ok_or_else(|| BuildError::missing_field("bulk"))?,
        })
    }
}
