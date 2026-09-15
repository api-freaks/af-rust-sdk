pub use crate::prelude::*;

/// Network information for the IP.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeolocationLookupV2ResponseNetwork {
    /// Network access type classification (e.g., DSL, Cable, Mobile, 5G) when available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// Network prefix in CIDR notation that contains the IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    /// true if the IP is anycast (same IP announced from multiple locations).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anycast: Option<bool>,
}

impl GeolocationLookupV2ResponseNetwork {
    pub fn builder() -> GeolocationLookupV2ResponseNetworkBuilder {
        <GeolocationLookupV2ResponseNetworkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeolocationLookupV2ResponseNetworkBuilder {
    connection_type: Option<String>,
    route: Option<String>,
    is_anycast: Option<bool>,
}

impl GeolocationLookupV2ResponseNetworkBuilder {
    pub fn connection_type(mut self, value: impl Into<String>) -> Self {
        self.connection_type = Some(value.into());
        self
    }

    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    pub fn is_anycast(mut self, value: bool) -> Self {
        self.is_anycast = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeolocationLookupV2ResponseNetwork`].
    pub fn build(self) -> Result<GeolocationLookupV2ResponseNetwork, BuildError> {
        Ok(GeolocationLookupV2ResponseNetwork {
            connection_type: self.connection_type,
            route: self.route,
            is_anycast: self.is_anycast,
        })
    }
}
