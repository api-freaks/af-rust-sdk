pub use crate::prelude::*;

/// Query parameters for zipcode_search_by_radius
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ZipcodeSearchByRadiusQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ZipcodeSearchByRadiusRequestFormat>,
    /// Postal/Zip code to be used as the center point for the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Latitude coordinate for the base location.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub lat: Option<f64>,
    /// Longitude coordinate for the base location.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub long: Option<f64>,
    /// Country code in ISO 3166-1 alpha-2 format. Required only when using the code parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Search radius for the query. The maximum allowed values are: - 100 km - 100 mi - 109361 yd - 100000 m - 328084 ft - 3937007.75 in
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub radius: f64,
    /// Supported distance units are m, km, mi, ft, yd, in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<ZipcodeSearchByRadiusRequestUnit>,
    /// Page no. to retrieve paginated results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl ZipcodeSearchByRadiusQueryRequest {
    pub fn builder() -> ZipcodeSearchByRadiusQueryRequestBuilder {
        <ZipcodeSearchByRadiusQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ZipcodeSearchByRadiusQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<ZipcodeSearchByRadiusRequestFormat>,
    code: Option<String>,
    lat: Option<f64>,
    long: Option<f64>,
    country: Option<String>,
    radius: Option<f64>,
    unit: Option<ZipcodeSearchByRadiusRequestUnit>,
    page: Option<i64>,
}

impl ZipcodeSearchByRadiusQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: ZipcodeSearchByRadiusRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
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

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn radius(mut self, value: f64) -> Self {
        self.radius = Some(value);
        self
    }

    pub fn unit(mut self, value: ZipcodeSearchByRadiusRequestUnit) -> Self {
        self.unit = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ZipcodeSearchByRadiusQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](ZipcodeSearchByRadiusQueryRequestBuilder::api_key)
    /// - [`radius`](ZipcodeSearchByRadiusQueryRequestBuilder::radius)
    pub fn build(self) -> Result<ZipcodeSearchByRadiusQueryRequest, BuildError> {
        Ok(ZipcodeSearchByRadiusQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            code: self.code,
            lat: self.lat,
            long: self.long,
            country: self.country,
            radius: self
                .radius
                .ok_or_else(|| BuildError::missing_field("radius"))?,
            unit: self.unit,
            page: self.page,
        })
    }
}
