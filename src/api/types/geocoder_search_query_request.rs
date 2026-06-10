pub use crate::prelude::*;

/// Query parameters for geocoder_search
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GeocoderSearchQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<GeocoderSearchRequestFormat>,
    /// Free-form search query, e.g. Wembley Stadium, London
    #[serde(default)]
    pub query: String,
    /// Max number of results to return (1–40). May return fewer if matches are weak.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Minimum latitude for the viewbox. Must be ≤ max_lat and between -90 and 90.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub min_lat: Option<f64>,
    /// Maximum latitude for the viewbox. Must be ≥ min_lat and between -90 and 90.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub max_lat: Option<f64>,
    /// Minimum longitude for the viewbox. Must be ≤ max_lon and between -180 and 180.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub min_lon: Option<f64>,
    /// Maximum longitude for the viewbox. Must be ≥ min_lon and between -180 and 180.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub max_lon: Option<f64>,
}

impl GeocoderSearchQueryRequest {
    pub fn builder() -> GeocoderSearchQueryRequestBuilder {
        <GeocoderSearchQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeocoderSearchQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<GeocoderSearchRequestFormat>,
    query: Option<String>,
    limit: Option<i64>,
    min_lat: Option<f64>,
    max_lat: Option<f64>,
    min_lon: Option<f64>,
    max_lon: Option<f64>,
}

impl GeocoderSearchQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: GeocoderSearchRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn min_lat(mut self, value: f64) -> Self {
        self.min_lat = Some(value);
        self
    }

    pub fn max_lat(mut self, value: f64) -> Self {
        self.max_lat = Some(value);
        self
    }

    pub fn min_lon(mut self, value: f64) -> Self {
        self.min_lon = Some(value);
        self
    }

    pub fn max_lon(mut self, value: f64) -> Self {
        self.max_lon = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeocoderSearchQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](GeocoderSearchQueryRequestBuilder::api_key)
    /// - [`query`](GeocoderSearchQueryRequestBuilder::query)
    pub fn build(self) -> Result<GeocoderSearchQueryRequest, BuildError> {
        Ok(GeocoderSearchQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            query: self
                .query
                .ok_or_else(|| BuildError::missing_field("query"))?,
            limit: self.limit,
            min_lat: self.min_lat,
            max_lat: self.max_lat,
            min_lon: self.min_lon,
            max_lon: self.max_lon,
        })
    }
}
