pub use crate::prelude::*;

/// Query parameters for zipcode_lookup
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ZipcodeLookupQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ZipcodeLookupRequestFormat>,
    /// Comma separated list of postal / zip codes. Max. 100 values.
    #[serde(default)]
    pub code: String,
    /// Country code in ISO 3166-1 alpha-2 format. If not provided, search results will be returned from all countries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl ZipcodeLookupQueryRequest {
    pub fn builder() -> ZipcodeLookupQueryRequestBuilder {
        <ZipcodeLookupQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ZipcodeLookupQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<ZipcodeLookupRequestFormat>,
    code: Option<String>,
    country: Option<String>,
}

impl ZipcodeLookupQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: ZipcodeLookupRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ZipcodeLookupQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](ZipcodeLookupQueryRequestBuilder::api_key)
    /// - [`code`](ZipcodeLookupQueryRequestBuilder::code)
    pub fn build(self) -> Result<ZipcodeLookupQueryRequest, BuildError> {
        Ok(ZipcodeLookupQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            country: self.country,
        })
    }
}
