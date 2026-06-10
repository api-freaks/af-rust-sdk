pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeolocationLookupResponseAbuseItem {
    /// The IP range of the abuse contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    /// The country code of the abuse contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The handle of the abuse contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    /// The name of the abuse contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The organization of the abuse contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// The role of the abuse contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// The kind of the abuse contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The address of the abuse contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The list of email addresses of the abuse contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
    /// The list of phone numbers of the abuse contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<String>>,
}

impl GeolocationLookupResponseAbuseItem {
    pub fn builder() -> GeolocationLookupResponseAbuseItemBuilder {
        <GeolocationLookupResponseAbuseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeolocationLookupResponseAbuseItemBuilder {
    route: Option<String>,
    country: Option<String>,
    handle: Option<String>,
    name: Option<String>,
    organization: Option<String>,
    role: Option<String>,
    kind: Option<String>,
    address: Option<String>,
    emails: Option<Vec<String>>,
    phone_numbers: Option<Vec<String>>,
}

impl GeolocationLookupResponseAbuseItemBuilder {
    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn handle(mut self, value: impl Into<String>) -> Self {
        self.handle = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn organization(mut self, value: impl Into<String>) -> Self {
        self.organization = Some(value.into());
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(value.into());
        self
    }

    pub fn emails(mut self, value: Vec<String>) -> Self {
        self.emails = Some(value);
        self
    }

    pub fn phone_numbers(mut self, value: Vec<String>) -> Self {
        self.phone_numbers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeolocationLookupResponseAbuseItem`].
    pub fn build(self) -> Result<GeolocationLookupResponseAbuseItem, BuildError> {
        Ok(GeolocationLookupResponseAbuseItem {
            route: self.route,
            country: self.country,
            handle: self.handle,
            name: self.name,
            organization: self.organization,
            role: self.role,
            kind: self.kind,
            address: self.address,
            emails: self.emails,
            phone_numbers: self.phone_numbers,
        })
    }
}
