pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainWhoisLookupResponseRegistrantContact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailing_address: Option<String>,
}

impl DomainWhoisLookupResponseRegistrantContact {
    pub fn builder() -> DomainWhoisLookupResponseRegistrantContactBuilder {
        <DomainWhoisLookupResponseRegistrantContactBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainWhoisLookupResponseRegistrantContactBuilder {
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

impl DomainWhoisLookupResponseRegistrantContactBuilder {
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

    /// Consumes the builder and constructs a [`DomainWhoisLookupResponseRegistrantContact`].
    pub fn build(self) -> Result<DomainWhoisLookupResponseRegistrantContact, BuildError> {
        Ok(DomainWhoisLookupResponseRegistrantContact {
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
        })
    }
}
