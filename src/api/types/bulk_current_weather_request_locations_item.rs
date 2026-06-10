pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkCurrentWeatherRequestLocationsItem {
    /// City name, place name, or full address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Latitude
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub lat: Option<f64>,
    /// Longitude
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub long: Option<f64>,
    /// IP address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

impl BulkCurrentWeatherRequestLocationsItem {
    pub fn builder() -> BulkCurrentWeatherRequestLocationsItemBuilder {
        <BulkCurrentWeatherRequestLocationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkCurrentWeatherRequestLocationsItemBuilder {
    location: Option<String>,
    lat: Option<f64>,
    long: Option<f64>,
    ip: Option<String>,
}

impl BulkCurrentWeatherRequestLocationsItemBuilder {
    pub fn location(mut self, value: impl Into<String>) -> Self {
        self.location = Some(value.into());
        self
    }

    pub fn lat(mut self, value: f64) -> Self {
        self.lat = Some(value);
        self
    }

    pub fn long(mut self, value: f64) -> Self {
        self.long = Some(value);
        self
    }

    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkCurrentWeatherRequestLocationsItem`].
    pub fn build(self) -> Result<BulkCurrentWeatherRequestLocationsItem, BuildError> {
        Ok(BulkCurrentWeatherRequestLocationsItem {
            location: self.location,
            lat: self.lat,
            long: self.long,
            ip: self.ip,
        })
    }
}
