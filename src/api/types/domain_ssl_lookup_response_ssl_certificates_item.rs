pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainSslLookupResponseSslCertificatesItem {
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
    pub subject: DomainSslLookupResponseSslCertificatesItemSubject,
    #[serde(default)]
    pub issuer: DomainSslLookupResponseSslCertificatesItemIssuer,
    #[serde(rename = "publicKey")]
    #[serde(default)]
    pub public_key: DomainSslLookupResponseSslCertificatesItemPublicKey,
    #[serde(default)]
    pub extensions: DomainSslLookupResponseSslCertificatesItemExtensions,
    /// Raw certificate in PEM format
    #[serde(rename = "pemRaw")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pem_raw: Option<String>,
}

impl DomainSslLookupResponseSslCertificatesItem {
    pub fn builder() -> DomainSslLookupResponseSslCertificatesItemBuilder {
        <DomainSslLookupResponseSslCertificatesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainSslLookupResponseSslCertificatesItemBuilder {
    chain_order: Option<String>,
    authentication_type: Option<String>,
    validity_start_date: Option<String>,
    validity_end_date: Option<String>,
    serial_number: Option<String>,
    signature_algorithm: Option<String>,
    subject: Option<DomainSslLookupResponseSslCertificatesItemSubject>,
    issuer: Option<DomainSslLookupResponseSslCertificatesItemIssuer>,
    public_key: Option<DomainSslLookupResponseSslCertificatesItemPublicKey>,
    extensions: Option<DomainSslLookupResponseSslCertificatesItemExtensions>,
    pem_raw: Option<String>,
}

impl DomainSslLookupResponseSslCertificatesItemBuilder {
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

    pub fn subject(mut self, value: DomainSslLookupResponseSslCertificatesItemSubject) -> Self {
        self.subject = Some(value);
        self
    }

    pub fn issuer(mut self, value: DomainSslLookupResponseSslCertificatesItemIssuer) -> Self {
        self.issuer = Some(value);
        self
    }

    pub fn public_key(
        mut self,
        value: DomainSslLookupResponseSslCertificatesItemPublicKey,
    ) -> Self {
        self.public_key = Some(value);
        self
    }

    pub fn extensions(
        mut self,
        value: DomainSslLookupResponseSslCertificatesItemExtensions,
    ) -> Self {
        self.extensions = Some(value);
        self
    }

    pub fn pem_raw(mut self, value: impl Into<String>) -> Self {
        self.pem_raw = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainSslLookupResponseSslCertificatesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chain_order`](DomainSslLookupResponseSslCertificatesItemBuilder::chain_order)
    /// - [`authentication_type`](DomainSslLookupResponseSslCertificatesItemBuilder::authentication_type)
    /// - [`validity_start_date`](DomainSslLookupResponseSslCertificatesItemBuilder::validity_start_date)
    /// - [`validity_end_date`](DomainSslLookupResponseSslCertificatesItemBuilder::validity_end_date)
    /// - [`serial_number`](DomainSslLookupResponseSslCertificatesItemBuilder::serial_number)
    /// - [`signature_algorithm`](DomainSslLookupResponseSslCertificatesItemBuilder::signature_algorithm)
    /// - [`subject`](DomainSslLookupResponseSslCertificatesItemBuilder::subject)
    /// - [`issuer`](DomainSslLookupResponseSslCertificatesItemBuilder::issuer)
    /// - [`public_key`](DomainSslLookupResponseSslCertificatesItemBuilder::public_key)
    /// - [`extensions`](DomainSslLookupResponseSslCertificatesItemBuilder::extensions)
    pub fn build(self) -> Result<DomainSslLookupResponseSslCertificatesItem, BuildError> {
        Ok(DomainSslLookupResponseSslCertificatesItem {
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
