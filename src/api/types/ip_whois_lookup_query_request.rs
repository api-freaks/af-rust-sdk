pub use crate::prelude::*;

/// Query parameters for ip_whois_lookup
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IpWhoisLookupQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<IpWhoisLookupRequestFormat>,
    /// The IP address (IPv4 or IPv6) for which WHOIS data is requested.
    #[serde(default)]
    pub ip: String,
}

impl IpWhoisLookupQueryRequest {
    pub fn builder() -> IpWhoisLookupQueryRequestBuilder {
        <IpWhoisLookupQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IpWhoisLookupQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<IpWhoisLookupRequestFormat>,
    ip: Option<String>,
}

impl IpWhoisLookupQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: IpWhoisLookupRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IpWhoisLookupQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](IpWhoisLookupQueryRequestBuilder::api_key)
    /// - [`ip`](IpWhoisLookupQueryRequestBuilder::ip)
    pub fn build(self) -> Result<IpWhoisLookupQueryRequest, BuildError> {
        Ok(IpWhoisLookupQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            ip: self.ip.ok_or_else(|| BuildError::missing_field("ip"))?,
        })
    }
}
