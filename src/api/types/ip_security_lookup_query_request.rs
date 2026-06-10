pub use crate::prelude::*;

/// Query parameters for ip_security_lookup
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IpSecurityLookupQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<IpSecurityLookupRequestFormat>,
    /// A valid IPv4 or IPv6 address to look up. If omitted, the API uses the public IP of the requesting client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Comma-separated list of fields to return. Supports dot notation (e.g. security.threat_score).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    /// Comma-separated list of fields to remove from the response. Supports dot notation (e.g. security.is_tor).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludes: Option<String>,
}

impl IpSecurityLookupQueryRequest {
    pub fn builder() -> IpSecurityLookupQueryRequestBuilder {
        <IpSecurityLookupQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IpSecurityLookupQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<IpSecurityLookupRequestFormat>,
    ip: Option<String>,
    fields: Option<String>,
    excludes: Option<String>,
}

impl IpSecurityLookupQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: IpSecurityLookupRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn fields(mut self, value: impl Into<String>) -> Self {
        self.fields = Some(value.into());
        self
    }

    pub fn excludes(mut self, value: impl Into<String>) -> Self {
        self.excludes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IpSecurityLookupQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](IpSecurityLookupQueryRequestBuilder::api_key)
    pub fn build(self) -> Result<IpSecurityLookupQueryRequest, BuildError> {
        Ok(IpSecurityLookupQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            ip: self.ip,
            fields: self.fields,
            excludes: self.excludes,
        })
    }
}
