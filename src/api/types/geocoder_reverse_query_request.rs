pub use crate::prelude::*;

/// Query parameters for geocoder_reverse
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GeocoderReverseQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<GeocoderReverseRequestFormat>,
    /// WGS84 latitude value ranging from -90 to 90.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub lat: f64,
    /// WGS84 longitude value ranging from -180 to 180.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub lon: f64,
}

impl GeocoderReverseQueryRequest {
    pub fn builder() -> GeocoderReverseQueryRequestBuilder {
        <GeocoderReverseQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeocoderReverseQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<GeocoderReverseRequestFormat>,
    lat: Option<f64>,
    lon: Option<f64>,
}

impl GeocoderReverseQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: GeocoderReverseRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn lat(mut self, value: f64) -> Self {
        self.lat = Some(value);
        self
    }

    pub fn lon(mut self, value: f64) -> Self {
        self.lon = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeocoderReverseQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](GeocoderReverseQueryRequestBuilder::api_key)
    /// - [`lat`](GeocoderReverseQueryRequestBuilder::lat)
    /// - [`lon`](GeocoderReverseQueryRequestBuilder::lon)
    pub fn build(self) -> Result<GeocoderReverseQueryRequest, BuildError> {
        Ok(GeocoderReverseQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            lat: self.lat.ok_or_else(|| BuildError::missing_field("lat"))?,
            lon: self.lon.ok_or_else(|| BuildError::missing_field("lon"))?,
        })
    }
}
