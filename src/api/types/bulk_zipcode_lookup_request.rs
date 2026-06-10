pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkZipcodeLookupRequest {
    /// Comma separated list of postal / zip codes. Max. 100 values.
    #[serde(default)]
    pub codes: Vec<String>,
    /// Country code in ISO 3166-1 alpha-2 format. If not provided, search results will be returned from all countries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing)]
    pub format: Option<BulkZipcodeLookupRequestFormat>,
}

impl BulkZipcodeLookupRequest {
    pub fn builder() -> BulkZipcodeLookupRequestBuilder {
        <BulkZipcodeLookupRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkZipcodeLookupRequestBuilder {
    codes: Option<Vec<String>>,
    country: Option<String>,
    api_key: Option<String>,
    format: Option<BulkZipcodeLookupRequestFormat>,
}

impl BulkZipcodeLookupRequestBuilder {
    pub fn codes(mut self, value: Vec<String>) -> Self {
        self.codes = Some(value);
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: BulkZipcodeLookupRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkZipcodeLookupRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`codes`](BulkZipcodeLookupRequestBuilder::codes)
    /// - [`api_key`](BulkZipcodeLookupRequestBuilder::api_key)
    pub fn build(self) -> Result<BulkZipcodeLookupRequest, BuildError> {
        Ok(BulkZipcodeLookupRequest {
            codes: self
                .codes
                .ok_or_else(|| BuildError::missing_field("codes"))?,
            country: self.country,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
        })
    }
}
