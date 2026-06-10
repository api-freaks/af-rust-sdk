pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainSslLookupResponse {
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "queryTime")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub query_time: DateTime<FixedOffset>,
    #[serde(rename = "sslCertificates")]
    #[serde(default)]
    pub ssl_certificates: Vec<DomainSslLookupResponseSslCertificatesItem>,
    #[serde(rename = "sslRaw")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_raw: Option<String>,
}

impl DomainSslLookupResponse {
    pub fn builder() -> DomainSslLookupResponseBuilder {
        <DomainSslLookupResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainSslLookupResponseBuilder {
    domain_name: Option<String>,
    query_time: Option<DateTime<FixedOffset>>,
    ssl_certificates: Option<Vec<DomainSslLookupResponseSslCertificatesItem>>,
    ssl_raw: Option<String>,
}

impl DomainSslLookupResponseBuilder {
    pub fn domain_name(mut self, value: impl Into<String>) -> Self {
        self.domain_name = Some(value.into());
        self
    }

    pub fn query_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.query_time = Some(value);
        self
    }

    pub fn ssl_certificates(
        mut self,
        value: Vec<DomainSslLookupResponseSslCertificatesItem>,
    ) -> Self {
        self.ssl_certificates = Some(value);
        self
    }

    pub fn ssl_raw(mut self, value: impl Into<String>) -> Self {
        self.ssl_raw = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainSslLookupResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain_name`](DomainSslLookupResponseBuilder::domain_name)
    /// - [`query_time`](DomainSslLookupResponseBuilder::query_time)
    /// - [`ssl_certificates`](DomainSslLookupResponseBuilder::ssl_certificates)
    pub fn build(self) -> Result<DomainSslLookupResponse, BuildError> {
        Ok(DomainSslLookupResponse {
            domain_name: self
                .domain_name
                .ok_or_else(|| BuildError::missing_field("domain_name"))?,
            query_time: self
                .query_time
                .ok_or_else(|| BuildError::missing_field("query_time"))?,
            ssl_certificates: self
                .ssl_certificates
                .ok_or_else(|| BuildError::missing_field("ssl_certificates"))?,
            ssl_raw: self.ssl_raw,
        })
    }
}
