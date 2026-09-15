pub use crate::prelude::*;

/// A contact record (registrant, administrative, technical, billing, or reseller) published in the domain's WHOIS record.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactTechnicalContact {
    /// Contact ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Contact's full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Contact's organization or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// Street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    /// City.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// State or province.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Postal/ZIP code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
    /// Full country name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>,
    /// ISO 3166-1 alpha-2 country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// Contact email address, or a privacy-service redirect instruction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// Contact phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Contact fax number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,
    /// Full combined mailing address, when published as a single string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailing_address: Option<String>,
}

impl BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactTechnicalContact {
    pub fn builder(
    ) -> BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactTechnicalContactBuilder
    {
        <BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactTechnicalContactBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactTechnicalContactBuilder {
    id: Option<String>,
    name: Option<String>,
    company: Option<String>,
    street: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zip_code: Option<String>,
    country_name: Option<String>,
    country_code: Option<String>,
    email_address: Option<String>,
    phone: Option<String>,
    fax: Option<String>,
    mailing_address: Option<String>,
}

impl BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactTechnicalContactBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn company(mut self, value: impl Into<String>) -> Self {
        self.company = Some(value.into());
        self
    }

    pub fn street(mut self, value: impl Into<String>) -> Self {
        self.street = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn zip_code(mut self, value: impl Into<String>) -> Self {
        self.zip_code = Some(value.into());
        self
    }

    pub fn country_name(mut self, value: impl Into<String>) -> Self {
        self.country_name = Some(value.into());
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn email_address(mut self, value: impl Into<String>) -> Self {
        self.email_address = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn fax(mut self, value: impl Into<String>) -> Self {
        self.fax = Some(value.into());
        self
    }

    pub fn mailing_address(mut self, value: impl Into<String>) -> Self {
        self.mailing_address = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactTechnicalContact`].
    pub fn build(
        self,
    ) -> Result<
        BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactTechnicalContact,
        BuildError,
    > {
        Ok(
            BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItemAbuseContactTechnicalContact {
                id: self.id,
                name: self.name,
                company: self.company,
                street: self.street,
                city: self.city,
                state: self.state,
                zip_code: self.zip_code,
                country_name: self.country_name,
                country_code: self.country_code,
                email_address: self.email_address,
                phone: self.phone,
                fax: self.fax,
                mailing_address: self.mailing_address,
            },
        )
    }
}
