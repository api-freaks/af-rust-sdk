pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainSslChainLookupResponseSslCertificatesItemPublicKey {
    #[serde(rename = "keySize")]
    #[serde(default)]
    pub key_size: String,
    #[serde(rename = "keyAlgorithm")]
    #[serde(default)]
    pub key_algorithm: String,
    #[serde(rename = "pemRaw")]
    #[serde(default)]
    pub pem_raw: String,
}

impl DomainSslChainLookupResponseSslCertificatesItemPublicKey {
    pub fn builder() -> DomainSslChainLookupResponseSslCertificatesItemPublicKeyBuilder {
        <DomainSslChainLookupResponseSslCertificatesItemPublicKeyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainSslChainLookupResponseSslCertificatesItemPublicKeyBuilder {
    key_size: Option<String>,
    key_algorithm: Option<String>,
    pem_raw: Option<String>,
}

impl DomainSslChainLookupResponseSslCertificatesItemPublicKeyBuilder {
    pub fn key_size(mut self, value: impl Into<String>) -> Self {
        self.key_size = Some(value.into());
        self
    }

    pub fn key_algorithm(mut self, value: impl Into<String>) -> Self {
        self.key_algorithm = Some(value.into());
        self
    }

    pub fn pem_raw(mut self, value: impl Into<String>) -> Self {
        self.pem_raw = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainSslChainLookupResponseSslCertificatesItemPublicKey`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key_size`](DomainSslChainLookupResponseSslCertificatesItemPublicKeyBuilder::key_size)
    /// - [`key_algorithm`](DomainSslChainLookupResponseSslCertificatesItemPublicKeyBuilder::key_algorithm)
    /// - [`pem_raw`](DomainSslChainLookupResponseSslCertificatesItemPublicKeyBuilder::pem_raw)
    pub fn build(
        self,
    ) -> Result<DomainSslChainLookupResponseSslCertificatesItemPublicKey, BuildError> {
        Ok(DomainSslChainLookupResponseSslCertificatesItemPublicKey {
            key_size: self
                .key_size
                .ok_or_else(|| BuildError::missing_field("key_size"))?,
            key_algorithm: self
                .key_algorithm
                .ok_or_else(|| BuildError::missing_field("key_algorithm"))?,
            pem_raw: self
                .pem_raw
                .ok_or_else(|| BuildError::missing_field("pem_raw"))?,
        })
    }
}
