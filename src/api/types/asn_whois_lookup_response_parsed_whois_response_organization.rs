pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AsnWhoisLookupResponseParsedWhoisResponseOrganization {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    #[serde(rename = "addressCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl AsnWhoisLookupResponseParsedWhoisResponseOrganization {
    pub fn builder() -> AsnWhoisLookupResponseParsedWhoisResponseOrganizationBuilder {
        <AsnWhoisLookupResponseParsedWhoisResponseOrganizationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AsnWhoisLookupResponseParsedWhoisResponseOrganizationBuilder {
    handle: Option<String>,
    name: Option<String>,
    address: Option<Vec<String>>,
    street: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zip_code: Option<String>,
    country: Option<Vec<String>>,
    address_country: Option<String>,
    date_created: Option<NaiveDate>,
    date_updated: Option<NaiveDate>,
    source: Option<String>,
}

impl AsnWhoisLookupResponseParsedWhoisResponseOrganizationBuilder {
    pub fn handle(mut self, value: impl Into<String>) -> Self {
        self.handle = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn address(mut self, value: Vec<String>) -> Self {
        self.address = Some(value);
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

    pub fn country(mut self, value: Vec<String>) -> Self {
        self.country = Some(value);
        self
    }

    pub fn address_country(mut self, value: impl Into<String>) -> Self {
        self.address_country = Some(value.into());
        self
    }

    pub fn date_created(mut self, value: NaiveDate) -> Self {
        self.date_created = Some(value);
        self
    }

    pub fn date_updated(mut self, value: NaiveDate) -> Self {
        self.date_updated = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AsnWhoisLookupResponseParsedWhoisResponseOrganization`].
    pub fn build(
        self,
    ) -> Result<AsnWhoisLookupResponseParsedWhoisResponseOrganization, BuildError> {
        Ok(AsnWhoisLookupResponseParsedWhoisResponseOrganization {
            handle: self.handle,
            name: self.name,
            address: self.address,
            street: self.street,
            city: self.city,
            state: self.state,
            zip_code: self.zip_code,
            country: self.country,
            address_country: self.address_country,
            date_created: self.date_created,
            date_updated: self.date_updated,
            source: self.source,
        })
    }
}
