pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
// required fields kept strict for untagged-union matching (fix_rust_untagged_union_required.py)
pub struct DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanyName {
    pub num: i64,
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
    /// Deprecated: the API returns this field as `company_name`; read `company_name` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companyname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
}

impl DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanyName {
    pub fn builder() -> DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanyNameBuilder {
        <DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanyNameBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanyNameBuilder {
    num: Option<i64>,
    domain_name: Option<String>,
    create_date: Option<NaiveDate>,
    update_date: Option<NaiveDate>,
    expiry_date: Option<NaiveDate>,
    name: Option<String>,
    email: Option<String>,
    companyname: Option<String>,
    company_name: Option<String>,
}

impl DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanyNameBuilder {
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

    pub fn company_name(mut self, value: impl Into<String>) -> Self {
        self.company_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanyName`].
    /// This method will fail if any of the following fields are not set:
    /// - [`num`](DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanyNameBuilder::num)
    /// - [`domain_name`](DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanyNameBuilder::domain_name)
    pub fn build(
        self,
    ) -> Result<DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanyName, BuildError> {
        Ok(
            DomainWhoisReverseResponseWhoisDomainsHistoricalItemCompanyName {
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
                company_name: self.company_name,
            },
        )
    }
}
