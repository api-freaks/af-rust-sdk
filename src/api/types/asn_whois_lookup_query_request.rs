pub use crate::prelude::*;

/// Query parameters for asn_whois_lookup
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AsnWhoisLookupQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<AsnWhoisLookupRequestFormat>,
    /// The Autonomous System Number (ASN) to retrieve WHOIS data for. Can be prefixed with 'as' or not.
    #[serde(default)]
    pub asn: String,
}

impl AsnWhoisLookupQueryRequest {
    pub fn builder() -> AsnWhoisLookupQueryRequestBuilder {
        <AsnWhoisLookupQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AsnWhoisLookupQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<AsnWhoisLookupRequestFormat>,
    asn: Option<String>,
}

impl AsnWhoisLookupQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: AsnWhoisLookupRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn asn(mut self, value: impl Into<String>) -> Self {
        self.asn = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AsnWhoisLookupQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](AsnWhoisLookupQueryRequestBuilder::api_key)
    /// - [`asn`](AsnWhoisLookupQueryRequestBuilder::asn)
    pub fn build(self) -> Result<AsnWhoisLookupQueryRequest, BuildError> {
        Ok(AsnWhoisLookupQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            asn: self.asn.ok_or_else(|| BuildError::missing_field("asn"))?,
        })
    }
}
