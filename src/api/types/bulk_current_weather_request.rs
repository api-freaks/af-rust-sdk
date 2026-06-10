pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkCurrentWeatherRequest {
    /// Array of locations to fetch weather data for
    #[serde(default)]
    pub locations: Vec<BulkCurrentWeatherRequestLocationsItem>,
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
    /// Response format returned by the API.
    #[serde(skip_serializing)]
    pub format: Option<BulkCurrentWeatherRequestFormat>,
    /// Timezone for the results.
    #[serde(skip_serializing)]
    pub timezone: Option<String>,
}

impl BulkCurrentWeatherRequest {
    pub fn builder() -> BulkCurrentWeatherRequestBuilder {
        <BulkCurrentWeatherRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkCurrentWeatherRequestBuilder {
    locations: Option<Vec<BulkCurrentWeatherRequestLocationsItem>>,
    api_key: Option<String>,
    format: Option<BulkCurrentWeatherRequestFormat>,
    timezone: Option<String>,
}

impl BulkCurrentWeatherRequestBuilder {
    pub fn locations(mut self, value: Vec<BulkCurrentWeatherRequestLocationsItem>) -> Self {
        self.locations = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: BulkCurrentWeatherRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkCurrentWeatherRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`locations`](BulkCurrentWeatherRequestBuilder::locations)
    /// - [`api_key`](BulkCurrentWeatherRequestBuilder::api_key)
    pub fn build(self) -> Result<BulkCurrentWeatherRequest, BuildError> {
        Ok(BulkCurrentWeatherRequest {
            locations: self
                .locations
                .ok_or_else(|| BuildError::missing_field("locations"))?,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            timezone: self.timezone,
        })
    }
}
