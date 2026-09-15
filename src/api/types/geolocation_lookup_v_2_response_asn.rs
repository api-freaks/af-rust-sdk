pub use crate::prelude::*;

/// Autonomous System details for the IP.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeolocationLookupV2ResponseAsn {
    /// ASN identifier in AS<number> format associated with the IP's network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_number: Option<String>,
    /// ASN operator name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// ASN registration country as ISO 3166-1 alpha-2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// ASN category (ISP, HOSTING, BUSINESS, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// ASN operator domain name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// ASN allocation date in YYYY-MM-DD format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_allocated: Option<String>,
    /// Regional Internet Registry that allocated the ASN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rir: Option<String>,
}

impl GeolocationLookupV2ResponseAsn {
    pub fn builder() -> GeolocationLookupV2ResponseAsnBuilder {
        <GeolocationLookupV2ResponseAsnBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeolocationLookupV2ResponseAsnBuilder {
    as_number: Option<String>,
    organization: Option<String>,
    country: Option<String>,
    r#type: Option<String>,
    domain: Option<String>,
    date_allocated: Option<String>,
    rir: Option<String>,
}

impl GeolocationLookupV2ResponseAsnBuilder {
    pub fn as_number(mut self, value: impl Into<String>) -> Self {
        self.as_number = Some(value.into());
        self
    }

    pub fn organization(mut self, value: impl Into<String>) -> Self {
        self.organization = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn date_allocated(mut self, value: impl Into<String>) -> Self {
        self.date_allocated = Some(value.into());
        self
    }

    pub fn rir(mut self, value: impl Into<String>) -> Self {
        self.rir = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GeolocationLookupV2ResponseAsn`].
    pub fn build(self) -> Result<GeolocationLookupV2ResponseAsn, BuildError> {
        Ok(GeolocationLookupV2ResponseAsn {
            as_number: self.as_number,
            organization: self.organization,
            country: self.country,
            r#type: self.r#type,
            domain: self.domain,
            date_allocated: self.date_allocated,
            rir: self.rir,
        })
    }
}
