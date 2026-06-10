pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkIpSecurityLookupRequest {
    /// List of IP addresses to lookup
    #[serde(default)]
    pub ips: Vec<String>,
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip_serializing)]
    pub format: Option<BulkIpSecurityLookupRequestFormat>,
    /// Comma-separated list of fields to return. Supports dot notation (e.g. security.threat_score).
    #[serde(skip_serializing)]
    pub fields: Option<String>,
    /// Comma-separated list of fields to remove from the response. Supports dot notation (e.g. security.is_tor).
    #[serde(skip_serializing)]
    pub excludes: Option<String>,
}

impl BulkIpSecurityLookupRequest {
    pub fn builder() -> BulkIpSecurityLookupRequestBuilder {
        <BulkIpSecurityLookupRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkIpSecurityLookupRequestBuilder {
    ips: Option<Vec<String>>,
    api_key: Option<String>,
    format: Option<BulkIpSecurityLookupRequestFormat>,
    fields: Option<String>,
    excludes: Option<String>,
}

impl BulkIpSecurityLookupRequestBuilder {
    pub fn ips(mut self, value: Vec<String>) -> Self {
        self.ips = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: BulkIpSecurityLookupRequestFormat) -> Self {
        self.format = Some(value);
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

    /// Consumes the builder and constructs a [`BulkIpSecurityLookupRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ips`](BulkIpSecurityLookupRequestBuilder::ips)
    /// - [`api_key`](BulkIpSecurityLookupRequestBuilder::api_key)
    pub fn build(self) -> Result<BulkIpSecurityLookupRequest, BuildError> {
        Ok(BulkIpSecurityLookupRequest {
            ips: self.ips.ok_or_else(|| BuildError::missing_field("ips"))?,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            fields: self.fields,
            excludes: self.excludes,
        })
    }
}
