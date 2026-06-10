pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainWhoisHistoryResponseWhoisDomainsHistoricalItemTechnicalContact {
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
    pub emailaddress: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailingaddress: Option<String>,
}

impl DomainWhoisHistoryResponseWhoisDomainsHistoricalItemTechnicalContact {
    pub fn builder() -> DomainWhoisHistoryResponseWhoisDomainsHistoricalItemTechnicalContactBuilder
    {
        <DomainWhoisHistoryResponseWhoisDomainsHistoricalItemTechnicalContactBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainWhoisHistoryResponseWhoisDomainsHistoricalItemTechnicalContactBuilder {
    name: Option<String>,
    company: Option<String>,
    street: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zip_code: Option<String>,
    country_name: Option<String>,
    country_code: Option<String>,
    emailaddress: Option<String>,
    phone: Option<String>,
    fax: Option<String>,
    mailingaddress: Option<String>,
}

impl DomainWhoisHistoryResponseWhoisDomainsHistoricalItemTechnicalContactBuilder {
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

    pub fn emailaddress(mut self, value: impl Into<String>) -> Self {
        self.emailaddress = Some(value.into());
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

    pub fn mailingaddress(mut self, value: impl Into<String>) -> Self {
        self.mailingaddress = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainWhoisHistoryResponseWhoisDomainsHistoricalItemTechnicalContact`].
    pub fn build(
        self,
    ) -> Result<DomainWhoisHistoryResponseWhoisDomainsHistoricalItemTechnicalContact, BuildError>
    {
        Ok(
            DomainWhoisHistoryResponseWhoisDomainsHistoricalItemTechnicalContact {
                name: self.name,
                company: self.company,
                street: self.street,
                city: self.city,
                state: self.state,
                zip_code: self.zip_code,
                country_name: self.country_name,
                country_code: self.country_code,
                emailaddress: self.emailaddress,
                phone: self.phone,
                fax: self.fax,
                mailingaddress: self.mailingaddress,
            },
        )
    }
}
