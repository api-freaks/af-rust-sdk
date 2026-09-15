pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanyname {
    #[serde(default)]
    pub num: i64,
    #[serde(default)]
    pub domain_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companyname: Option<String>,
}

impl DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanyname {
    pub fn builder() -> DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanynameBuilder {
        <DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanynameBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanynameBuilder {
    num: Option<i64>,
    domain_name: Option<String>,
    create_date: Option<NaiveDate>,
    update_date: Option<NaiveDate>,
    expiry_date: Option<NaiveDate>,
    name: Option<String>,
    email: Option<String>,
    companyname: Option<String>,
}

impl DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanynameBuilder {
    pub fn num(mut self, value: i64) -> Self {
        self.num = Some(value);
        self
    }

    pub fn domain_name(mut self, value: impl Into<String>) -> Self {
        self.domain_name = Some(value.into());
        self
    }

    pub fn create_date(mut self, value: NaiveDate) -> Self {
        self.create_date = Some(value);
        self
    }

    pub fn update_date(mut self, value: NaiveDate) -> Self {
        self.update_date = Some(value);
        self
    }

    pub fn expiry_date(mut self, value: NaiveDate) -> Self {
        self.expiry_date = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn companyname(mut self, value: impl Into<String>) -> Self {
        self.companyname = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanyname`].
    /// This method will fail if any of the following fields are not set:
    /// - [`num`](DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanynameBuilder::num)
    /// - [`domain_name`](DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanynameBuilder::domain_name)
    pub fn build(
        self,
    ) -> Result<DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanyname, BuildError> {
        Ok(
            DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanyname {
                num: self.num.ok_or_else(|| BuildError::missing_field("num"))?,
                domain_name: self
                    .domain_name
                    .ok_or_else(|| BuildError::missing_field("domain_name"))?,
                create_date: self.create_date,
                update_date: self.update_date,
                expiry_date: self.expiry_date,
                name: self.name,
                email: self.email,
                companyname: self.companyname,
            },
        )
    }
}
