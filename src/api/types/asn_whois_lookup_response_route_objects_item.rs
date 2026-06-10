pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AsnWhoisLookupResponseRouteObjectsItem {
    #[serde(default)]
    pub route: String,
    #[serde(default)]
    pub origin: String,
    #[serde(rename = "originName")]
    #[serde(default)]
    pub origin_name: String,
    #[serde(default)]
    pub isp: String,
    #[serde(rename = "numberOfIps")]
    #[serde(default)]
    pub number_of_ips: i64,
}

impl AsnWhoisLookupResponseRouteObjectsItem {
    pub fn builder() -> AsnWhoisLookupResponseRouteObjectsItemBuilder {
        <AsnWhoisLookupResponseRouteObjectsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AsnWhoisLookupResponseRouteObjectsItemBuilder {
    route: Option<String>,
    origin: Option<String>,
    origin_name: Option<String>,
    isp: Option<String>,
    number_of_ips: Option<i64>,
}

impl AsnWhoisLookupResponseRouteObjectsItemBuilder {
    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    pub fn origin(mut self, value: impl Into<String>) -> Self {
        self.origin = Some(value.into());
        self
    }

    pub fn origin_name(mut self, value: impl Into<String>) -> Self {
        self.origin_name = Some(value.into());
        self
    }

    pub fn isp(mut self, value: impl Into<String>) -> Self {
        self.isp = Some(value.into());
        self
    }

    pub fn number_of_ips(mut self, value: i64) -> Self {
        self.number_of_ips = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AsnWhoisLookupResponseRouteObjectsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`route`](AsnWhoisLookupResponseRouteObjectsItemBuilder::route)
    /// - [`origin`](AsnWhoisLookupResponseRouteObjectsItemBuilder::origin)
    /// - [`origin_name`](AsnWhoisLookupResponseRouteObjectsItemBuilder::origin_name)
    /// - [`isp`](AsnWhoisLookupResponseRouteObjectsItemBuilder::isp)
    /// - [`number_of_ips`](AsnWhoisLookupResponseRouteObjectsItemBuilder::number_of_ips)
    pub fn build(self) -> Result<AsnWhoisLookupResponseRouteObjectsItem, BuildError> {
        Ok(AsnWhoisLookupResponseRouteObjectsItem {
            route: self
                .route
                .ok_or_else(|| BuildError::missing_field("route"))?,
            origin: self
                .origin
                .ok_or_else(|| BuildError::missing_field("origin"))?,
            origin_name: self
                .origin_name
                .ok_or_else(|| BuildError::missing_field("origin_name"))?,
            isp: self.isp.ok_or_else(|| BuildError::missing_field("isp"))?,
            number_of_ips: self
                .number_of_ips
                .ok_or_else(|| BuildError::missing_field("number_of_ips"))?,
        })
    }
}
