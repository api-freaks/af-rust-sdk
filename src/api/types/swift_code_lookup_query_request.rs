pub use crate::prelude::*;

/// Query parameters for swift_code_lookup
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SwiftCodeLookupQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Specify the desired response format. Options: 'json' (default) or 'xml'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<SwiftCodeLookupRequestFormat>,
    /// SWIFT/BIC code to lookup (must be 8 or 11 characters).
    #[serde(rename = "swiftCode")]
    #[serde(default)]
    pub swift_code: String,
}

impl SwiftCodeLookupQueryRequest {
    pub fn builder() -> SwiftCodeLookupQueryRequestBuilder {
        <SwiftCodeLookupQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SwiftCodeLookupQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<SwiftCodeLookupRequestFormat>,
    swift_code: Option<String>,
}

impl SwiftCodeLookupQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: SwiftCodeLookupRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn swift_code(mut self, value: impl Into<String>) -> Self {
        self.swift_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SwiftCodeLookupQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](SwiftCodeLookupQueryRequestBuilder::api_key)
    /// - [`swift_code`](SwiftCodeLookupQueryRequestBuilder::swift_code)
    pub fn build(self) -> Result<SwiftCodeLookupQueryRequest, BuildError> {
        Ok(SwiftCodeLookupQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            swift_code: self
                .swift_code
                .ok_or_else(|| BuildError::missing_field("swift_code"))?,
        })
    }
}
