pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeolocationLookupResponseNetworkAsn {
    /// The AS number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_number: Option<String>,
    /// The organization name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// The country code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The ASN name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn_name: Option<String>,
    /// The type of the ASN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The domain associated with the ASN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The date when the ASN was allocated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_allocated: Option<String>,
    /// The allocation status of the ASN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_status: Option<String>,
    /// The number of IPv4 routes associated with the ASN
    #[serde(rename = "num_of_ipv4_routes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_of_ipv4routes: Option<String>,
    /// The number of IPv6 routes associated with the ASN
    #[serde(rename = "num_of_ipv6_routes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_of_ipv6routes: Option<String>,
    /// The Regional Internet Registry (RIR) of the ASN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rir: Option<String>,
}

impl GeolocationLookupResponseNetworkAsn {
    pub fn builder() -> GeolocationLookupResponseNetworkAsnBuilder {
        <GeolocationLookupResponseNetworkAsnBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeolocationLookupResponseNetworkAsnBuilder {
    as_number: Option<String>,
    organization: Option<String>,
    country: Option<String>,
    asn_name: Option<String>,
    r#type: Option<String>,
    domain: Option<String>,
    date_allocated: Option<String>,
    allocation_status: Option<String>,
    num_of_ipv4routes: Option<String>,
    num_of_ipv6routes: Option<String>,
    rir: Option<String>,
}

impl GeolocationLookupResponseNetworkAsnBuilder {
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

    pub fn asn_name(mut self, value: impl Into<String>) -> Self {
        self.asn_name = Some(value.into());
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

    pub fn allocation_status(mut self, value: impl Into<String>) -> Self {
        self.allocation_status = Some(value.into());
        self
    }

    pub fn num_of_ipv4routes(mut self, value: impl Into<String>) -> Self {
        self.num_of_ipv4routes = Some(value.into());
        self
    }

    pub fn num_of_ipv6routes(mut self, value: impl Into<String>) -> Self {
        self.num_of_ipv6routes = Some(value.into());
        self
    }

    pub fn rir(mut self, value: impl Into<String>) -> Self {
        self.rir = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GeolocationLookupResponseNetworkAsn`].
    pub fn build(self) -> Result<GeolocationLookupResponseNetworkAsn, BuildError> {
        Ok(GeolocationLookupResponseNetworkAsn {
            as_number: self.as_number,
            organization: self.organization,
            country: self.country,
            asn_name: self.asn_name,
            r#type: self.r#type,
            domain: self.domain,
            date_allocated: self.date_allocated,
            allocation_status: self.allocation_status,
            num_of_ipv4routes: self.num_of_ipv4routes,
            num_of_ipv6routes: self.num_of_ipv6routes,
            rir: self.rir,
        })
    }
}
