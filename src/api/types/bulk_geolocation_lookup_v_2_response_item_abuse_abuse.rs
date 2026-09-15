pub use crate::prelude::*;

/// Abuse contact information for the IP.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkGeolocationLookupV2ResponseItemAbuseAbuse {
    /// Abuse-handling IP range in CIDR notation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    /// ISO 3166-1 alpha-2 country code of the abuse contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Display name for the abuse contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Organization name for the abuse contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// Contact type: group or individual.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Registered address of the organization owning the IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Abuse contact email addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
    /// Abuse contact phone numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<String>>,
}

impl BulkGeolocationLookupV2ResponseItemAbuseAbuse {
    pub fn builder() -> BulkGeolocationLookupV2ResponseItemAbuseAbuseBuilder {
        <BulkGeolocationLookupV2ResponseItemAbuseAbuseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkGeolocationLookupV2ResponseItemAbuseAbuseBuilder {
    route: Option<String>,
    country: Option<String>,
    name: Option<String>,
    organization: Option<String>,
    kind: Option<String>,
    address: Option<String>,
    emails: Option<Vec<String>>,
    phone_numbers: Option<Vec<String>>,
}

impl BulkGeolocationLookupV2ResponseItemAbuseAbuseBuilder {
    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
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

    /// Consumes the builder and constructs a [`BulkGeolocationLookupV2ResponseItemAbuseAbuse`].
    pub fn build(self) -> Result<BulkGeolocationLookupV2ResponseItemAbuseAbuse, BuildError> {
        Ok(BulkGeolocationLookupV2ResponseItemAbuseAbuse {
            route: self.route,
            country: self.country,
            name: self.name,
            organization: self.organization,
            kind: self.kind,
            address: self.address,
            emails: self.emails,
            phone_numbers: self.phone_numbers,
        })
    }
}
