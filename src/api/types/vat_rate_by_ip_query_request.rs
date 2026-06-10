pub use crate::prelude::*;

/// Query parameters for vat_rate_by_ip
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatRateByIpQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Specify the desired response format. Options: 'json' (default) or 'xml'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<VatRateByIpRequestFormat>,
    /// IPv4 or IPv6 address to look up VAT rate for. If omitted, the originating IP address will be used.
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

impl VatRateByIpQueryRequest {
    pub fn builder() -> VatRateByIpQueryRequestBuilder {
        <VatRateByIpQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatRateByIpQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<VatRateByIpRequestFormat>,
    ip_address: Option<String>,
}

impl VatRateByIpQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: VatRateByIpRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn ip_address(mut self, value: impl Into<String>) -> Self {
        self.ip_address = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VatRateByIpQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](VatRateByIpQueryRequestBuilder::api_key)
    pub fn build(self) -> Result<VatRateByIpQueryRequest, BuildError> {
        Ok(VatRateByIpQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            ip_address: self.ip_address,
        })
    }
}
