pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkUserAgentLookupRequest {
    /// List of user agent strings to parse
    #[serde(rename = "uaStrings")]
    #[serde(default)]
    pub ua_strings: Vec<String>,
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response
    #[serde(skip_serializing)]
    pub format: Option<BulkUserAgentLookupRequestFormat>,
}

impl BulkUserAgentLookupRequest {
    pub fn builder() -> BulkUserAgentLookupRequestBuilder {
        <BulkUserAgentLookupRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkUserAgentLookupRequestBuilder {
    ua_strings: Option<Vec<String>>,
    api_key: Option<String>,
    format: Option<BulkUserAgentLookupRequestFormat>,
}

impl BulkUserAgentLookupRequestBuilder {
    pub fn ua_strings(mut self, value: Vec<String>) -> Self {
        self.ua_strings = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: BulkUserAgentLookupRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkUserAgentLookupRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ua_strings`](BulkUserAgentLookupRequestBuilder::ua_strings)
    /// - [`api_key`](BulkUserAgentLookupRequestBuilder::api_key)
    pub fn build(self) -> Result<BulkUserAgentLookupRequest, BuildError> {
        Ok(BulkUserAgentLookupRequest {
            ua_strings: self
                .ua_strings
                .ok_or_else(|| BuildError::missing_field("ua_strings"))?,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
        })
    }
}
