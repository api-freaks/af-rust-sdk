pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainSslLookupResponseSslCertificatesItemSubject {
    #[serde(rename = "commonName")]
    #[serde(default)]
    pub common_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(rename = "organizationalUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "incCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inc_country: Option<String>,
    #[serde(rename = "incState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inc_state: Option<String>,
    #[serde(rename = "businessCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(rename = "postalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(rename = "serialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
}

impl DomainSslLookupResponseSslCertificatesItemSubject {
    pub fn builder() -> DomainSslLookupResponseSslCertificatesItemSubjectBuilder {
        <DomainSslLookupResponseSslCertificatesItemSubjectBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainSslLookupResponseSslCertificatesItemSubjectBuilder {
    common_name: Option<String>,
    organization: Option<String>,
    organizational_unit: Option<String>,
    locality: Option<String>,
    state: Option<String>,
    country: Option<String>,
    inc_country: Option<String>,
    inc_state: Option<String>,
    business_category: Option<String>,
    street: Option<String>,
    postal_code: Option<String>,
    serial_number: Option<String>,
}

impl DomainSslLookupResponseSslCertificatesItemSubjectBuilder {
    pub fn common_name(mut self, value: impl Into<String>) -> Self {
        self.common_name = Some(value.into());
        self
    }

    pub fn organization(mut self, value: impl Into<String>) -> Self {
        self.organization = Some(value.into());
        self
    }

    pub fn organizational_unit(mut self, value: impl Into<String>) -> Self {
        self.organizational_unit = Some(value.into());
        self
    }

    pub fn locality(mut self, value: impl Into<String>) -> Self {
        self.locality = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn inc_country(mut self, value: impl Into<String>) -> Self {
        self.inc_country = Some(value.into());
        self
    }

    pub fn inc_state(mut self, value: impl Into<String>) -> Self {
        self.inc_state = Some(value.into());
        self
    }

    pub fn business_category(mut self, value: impl Into<String>) -> Self {
        self.business_category = Some(value.into());
        self
    }

    pub fn street(mut self, value: impl Into<String>) -> Self {
        self.street = Some(value.into());
        self
    }

    pub fn postal_code(mut self, value: impl Into<String>) -> Self {
        self.postal_code = Some(value.into());
        self
    }

    pub fn serial_number(mut self, value: impl Into<String>) -> Self {
        self.serial_number = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainSslLookupResponseSslCertificatesItemSubject`].
    /// This method will fail if any of the following fields are not set:
    /// - [`common_name`](DomainSslLookupResponseSslCertificatesItemSubjectBuilder::common_name)
    pub fn build(self) -> Result<DomainSslLookupResponseSslCertificatesItemSubject, BuildError> {
        Ok(DomainSslLookupResponseSslCertificatesItemSubject {
            common_name: self
                .common_name
                .ok_or_else(|| BuildError::missing_field("common_name"))?,
            organization: self.organization,
            organizational_unit: self.organizational_unit,
            locality: self.locality,
            state: self.state,
            country: self.country,
            inc_country: self.inc_country,
            inc_state: self.inc_state,
            business_category: self.business_category,
            street: self.street,
            postal_code: self.postal_code,
            serial_number: self.serial_number,
        })
    }
}
