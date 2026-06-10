pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainSslChainLookupResponseSslCertificatesItem {
    #[serde(rename = "chainOrder")]
    #[serde(default)]
    pub chain_order: String,
    #[serde(rename = "authenticationType")]
    #[serde(default)]
    pub authentication_type: String,
    #[serde(rename = "validityStartDate")]
    #[serde(default)]
    pub validity_start_date: String,
    #[serde(rename = "validityEndDate")]
    #[serde(default)]
    pub validity_end_date: String,
    #[serde(rename = "serialNumber")]
    #[serde(default)]
    pub serial_number: String,
    #[serde(rename = "signatureAlgorithm")]
    #[serde(default)]
    pub signature_algorithm: String,
    #[serde(default)]
    pub subject: DomainSslChainLookupResponseSslCertificatesItemSubject,
    #[serde(default)]
    pub issuer: DomainSslChainLookupResponseSslCertificatesItemIssuer,
    #[serde(rename = "publicKey")]
    #[serde(default)]
    pub public_key: DomainSslChainLookupResponseSslCertificatesItemPublicKey,
    #[serde(default)]
    pub extensions: DomainSslChainLookupResponseSslCertificatesItemExtensions,
    /// Raw certificate in PEM format
    #[serde(rename = "pemRaw")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pem_raw: Option<String>,
}

impl DomainSslChainLookupResponseSslCertificatesItem {
    pub fn builder() -> DomainSslChainLookupResponseSslCertificatesItemBuilder {
        <DomainSslChainLookupResponseSslCertificatesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainSslChainLookupResponseSslCertificatesItemBuilder {
    chain_order: Option<String>,
    authentication_type: Option<String>,
    validity_start_date: Option<String>,
    validity_end_date: Option<String>,
    serial_number: Option<String>,
    signature_algorithm: Option<String>,
    subject: Option<DomainSslChainLookupResponseSslCertificatesItemSubject>,
    issuer: Option<DomainSslChainLookupResponseSslCertificatesItemIssuer>,
    public_key: Option<DomainSslChainLookupResponseSslCertificatesItemPublicKey>,
    extensions: Option<DomainSslChainLookupResponseSslCertificatesItemExtensions>,
    pem_raw: Option<String>,
}

impl DomainSslChainLookupResponseSslCertificatesItemBuilder {
    pub fn chain_order(mut self, value: impl Into<String>) -> Self {
        self.chain_order = Some(value.into());
        self
    }

    pub fn authentication_type(mut self, value: impl Into<String>) -> Self {
        self.authentication_type = Some(value.into());
        self
    }

    pub fn validity_start_date(mut self, value: impl Into<String>) -> Self {
        self.validity_start_date = Some(value.into());
        self
    }

    pub fn validity_end_date(mut self, value: impl Into<String>) -> Self {
        self.validity_end_date = Some(value.into());
        self
    }

    pub fn serial_number(mut self, value: impl Into<String>) -> Self {
        self.serial_number = Some(value.into());
        self
    }

    pub fn signature_algorithm(mut self, value: impl Into<String>) -> Self {
        self.signature_algorithm = Some(value.into());
        self
    }

    pub fn subject(
        mut self,
        value: DomainSslChainLookupResponseSslCertificatesItemSubject,
    ) -> Self {
        self.subject = Some(value);
        self
    }

    pub fn issuer(mut self, value: DomainSslChainLookupResponseSslCertificatesItemIssuer) -> Self {
        self.issuer = Some(value);
        self
    }

    pub fn public_key(
        mut self,
        value: DomainSslChainLookupResponseSslCertificatesItemPublicKey,
    ) -> Self {
        self.public_key = Some(value);
        self
    }

    pub fn extensions(
        mut self,
        value: DomainSslChainLookupResponseSslCertificatesItemExtensions,
    ) -> Self {
        self.extensions = Some(value);
        self
    }

    pub fn pem_raw(mut self, value: impl Into<String>) -> Self {
        self.pem_raw = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainSslChainLookupResponseSslCertificatesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chain_order`](DomainSslChainLookupResponseSslCertificatesItemBuilder::chain_order)
    /// - [`authentication_type`](DomainSslChainLookupResponseSslCertificatesItemBuilder::authentication_type)
    /// - [`validity_start_date`](DomainSslChainLookupResponseSslCertificatesItemBuilder::validity_start_date)
    /// - [`validity_end_date`](DomainSslChainLookupResponseSslCertificatesItemBuilder::validity_end_date)
    /// - [`serial_number`](DomainSslChainLookupResponseSslCertificatesItemBuilder::serial_number)
    /// - [`signature_algorithm`](DomainSslChainLookupResponseSslCertificatesItemBuilder::signature_algorithm)
    /// - [`subject`](DomainSslChainLookupResponseSslCertificatesItemBuilder::subject)
    /// - [`issuer`](DomainSslChainLookupResponseSslCertificatesItemBuilder::issuer)
    /// - [`public_key`](DomainSslChainLookupResponseSslCertificatesItemBuilder::public_key)
    /// - [`extensions`](DomainSslChainLookupResponseSslCertificatesItemBuilder::extensions)
    pub fn build(self) -> Result<DomainSslChainLookupResponseSslCertificatesItem, BuildError> {
        Ok(DomainSslChainLookupResponseSslCertificatesItem {
            chain_order: self
                .chain_order
                .ok_or_else(|| BuildError::missing_field("chain_order"))?,
            authentication_type: self
                .authentication_type
                .ok_or_else(|| BuildError::missing_field("authentication_type"))?,
            validity_start_date: self
                .validity_start_date
                .ok_or_else(|| BuildError::missing_field("validity_start_date"))?,
            validity_end_date: self
                .validity_end_date
                .ok_or_else(|| BuildError::missing_field("validity_end_date"))?,
            serial_number: self
                .serial_number
                .ok_or_else(|| BuildError::missing_field("serial_number"))?,
            signature_algorithm: self
                .signature_algorithm
                .ok_or_else(|| BuildError::missing_field("signature_algorithm"))?,
            subject: self
                .subject
                .ok_or_else(|| BuildError::missing_field("subject"))?,
            issuer: self
                .issuer
                .ok_or_else(|| BuildError::missing_field("issuer"))?,
            public_key: self
                .public_key
                .ok_or_else(|| BuildError::missing_field("public_key"))?,
            extensions: self
                .extensions
                .ok_or_else(|| BuildError::missing_field("extensions"))?,
            pem_raw: self.pem_raw,
        })
    }
}
