pub use crate::prelude::*;

/// Query parameters for swift_code_find
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SwiftCodeFindQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Specify the desired response format. Options: 'json' (default) or 'xml'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<SwiftCodeFindRequestFormat>,
    /// Country name (accepts full name, e.g., Pakistan, United States). If only the country parameter is supplied, lists all banks in the country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Bank name (upper case) used to filter SWIFT codes. Should be used together with the country parameter. If only country and bank are provided (without city), returns the list of cities for that bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<String>,
    /// Gives SWIFT codes for a bank. Optionally specify the city (upper case) to narrow results to a specific city for that bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
}

impl SwiftCodeFindQueryRequest {
    pub fn builder() -> SwiftCodeFindQueryRequestBuilder {
        <SwiftCodeFindQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SwiftCodeFindQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<SwiftCodeFindRequestFormat>,
    country: Option<String>,
    bank: Option<String>,
    city: Option<String>,
}

impl SwiftCodeFindQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: SwiftCodeFindRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn bank(mut self, value: impl Into<String>) -> Self {
        self.bank = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SwiftCodeFindQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](SwiftCodeFindQueryRequestBuilder::api_key)
    pub fn build(self) -> Result<SwiftCodeFindQueryRequest, BuildError> {
        Ok(SwiftCodeFindQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            country: self.country,
            bank: self.bank,
            city: self.city,
        })
    }
}
