pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AsnWhoisLookupResponse {
    #[serde(rename = "asNumber")]
    #[serde(default)]
    pub as_number: String,
    #[serde(rename = "asName")]
    #[serde(default)]
    pub as_name: String,
    #[serde(rename = "orgName")]
    #[serde(default)]
    pub org_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "orgHandle")]
    #[serde(default)]
    pub org_handle: String,
    #[serde(default)]
    pub country: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(rename = "allocationStatus")]
    #[serde(default)]
    pub allocation_status: String,
    #[serde(rename = "numOfIPv4Routes")]
    #[serde(default)]
    pub num_of_i_pv4routes: String,
    #[serde(rename = "numOfIPv6Routes")]
    #[serde(default)]
    pub num_of_i_pv6routes: String,
    #[serde(rename = "whoisHost")]
    #[serde(default)]
    pub whois_host: String,
    #[serde(rename = "dateAllocated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_allocated: Option<NaiveDate>,
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "routeObjects")]
    #[serde(default)]
    pub route_objects: Vec<AsnWhoisLookupResponseRouteObjectsItem>,
    #[serde(rename = "whoisResponse")]
    #[serde(default)]
    pub whois_response: String,
    #[serde(default)]
    pub downstreams: Vec<AsnWhoisLookupResponseDownstreamsItem>,
    #[serde(rename = "parsedWhoisResponse")]
    #[serde(default)]
    pub parsed_whois_response: AsnWhoisLookupResponseParsedWhoisResponse,
    #[serde(default)]
    pub upstreams: Vec<AsnWhoisLookupResponseUpstreamsItem>,
    #[serde(default)]
    pub peers: Vec<AsnWhoisLookupResponsePeersItem>,
    #[serde(default)]
    pub contacts: AsnWhoisLookupResponseContacts,
    #[serde(rename = "legacyRoutes")]
    #[serde(default)]
    pub legacy_routes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whois_raw_response: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r_whois_raw_response: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_time: Option<String>,
}

impl AsnWhoisLookupResponse {
    pub fn builder() -> AsnWhoisLookupResponseBuilder {
        <AsnWhoisLookupResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AsnWhoisLookupResponseBuilder {
    as_number: Option<String>,
    as_name: Option<String>,
    org_name: Option<String>,
    description: Option<String>,
    org_handle: Option<String>,
    country: Option<String>,
    domain: Option<String>,
    website: Option<String>,
    allocation_status: Option<String>,
    num_of_i_pv4routes: Option<String>,
    num_of_i_pv6routes: Option<String>,
    whois_host: Option<String>,
    date_allocated: Option<NaiveDate>,
    r#type: Option<String>,
    route_objects: Option<Vec<AsnWhoisLookupResponseRouteObjectsItem>>,
    whois_response: Option<String>,
    downstreams: Option<Vec<AsnWhoisLookupResponseDownstreamsItem>>,
    parsed_whois_response: Option<AsnWhoisLookupResponseParsedWhoisResponse>,
    upstreams: Option<Vec<AsnWhoisLookupResponseUpstreamsItem>>,
    peers: Option<Vec<AsnWhoisLookupResponsePeersItem>>,
    contacts: Option<AsnWhoisLookupResponseContacts>,
    legacy_routes: Option<Vec<String>>,
    whois_raw_response: Option<String>,
    r_whois_raw_response: Option<String>,
    query_time: Option<String>,
}

impl AsnWhoisLookupResponseBuilder {
    pub fn as_number(mut self, value: impl Into<String>) -> Self {
        self.as_number = Some(value.into());
        self
    }

    pub fn as_name(mut self, value: impl Into<String>) -> Self {
        self.as_name = Some(value.into());
        self
    }

    pub fn org_name(mut self, value: impl Into<String>) -> Self {
        self.org_name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn org_handle(mut self, value: impl Into<String>) -> Self {
        self.org_handle = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn website(mut self, value: impl Into<String>) -> Self {
        self.website = Some(value.into());
        self
    }

    pub fn allocation_status(mut self, value: impl Into<String>) -> Self {
        self.allocation_status = Some(value.into());
        self
    }

    pub fn num_of_i_pv4routes(mut self, value: impl Into<String>) -> Self {
        self.num_of_i_pv4routes = Some(value.into());
        self
    }

    pub fn num_of_i_pv6routes(mut self, value: impl Into<String>) -> Self {
        self.num_of_i_pv6routes = Some(value.into());
        self
    }

    pub fn whois_host(mut self, value: impl Into<String>) -> Self {
        self.whois_host = Some(value.into());
        self
    }

    pub fn date_allocated(mut self, value: NaiveDate) -> Self {
        self.date_allocated = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn route_objects(mut self, value: Vec<AsnWhoisLookupResponseRouteObjectsItem>) -> Self {
        self.route_objects = Some(value);
        self
    }

    pub fn whois_response(mut self, value: impl Into<String>) -> Self {
        self.whois_response = Some(value.into());
        self
    }

    pub fn downstreams(mut self, value: Vec<AsnWhoisLookupResponseDownstreamsItem>) -> Self {
        self.downstreams = Some(value);
        self
    }

    pub fn parsed_whois_response(
        mut self,
        value: AsnWhoisLookupResponseParsedWhoisResponse,
    ) -> Self {
        self.parsed_whois_response = Some(value);
        self
    }

    pub fn upstreams(mut self, value: Vec<AsnWhoisLookupResponseUpstreamsItem>) -> Self {
        self.upstreams = Some(value);
        self
    }

    pub fn peers(mut self, value: Vec<AsnWhoisLookupResponsePeersItem>) -> Self {
        self.peers = Some(value);
        self
    }

    pub fn contacts(mut self, value: AsnWhoisLookupResponseContacts) -> Self {
        self.contacts = Some(value);
        self
    }

    pub fn legacy_routes(mut self, value: Vec<String>) -> Self {
        self.legacy_routes = Some(value);
        self
    }

    pub fn whois_raw_response(mut self, value: impl Into<String>) -> Self {
        self.whois_raw_response = Some(value.into());
        self
    }

    pub fn r_whois_raw_response(mut self, value: impl Into<String>) -> Self {
        self.r_whois_raw_response = Some(value.into());
        self
    }

    pub fn query_time(mut self, value: impl Into<String>) -> Self {
        self.query_time = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AsnWhoisLookupResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`as_number`](AsnWhoisLookupResponseBuilder::as_number)
    /// - [`as_name`](AsnWhoisLookupResponseBuilder::as_name)
    /// - [`org_name`](AsnWhoisLookupResponseBuilder::org_name)
    /// - [`org_handle`](AsnWhoisLookupResponseBuilder::org_handle)
    /// - [`country`](AsnWhoisLookupResponseBuilder::country)
    /// - [`allocation_status`](AsnWhoisLookupResponseBuilder::allocation_status)
    /// - [`num_of_i_pv4routes`](AsnWhoisLookupResponseBuilder::num_of_i_pv4routes)
    /// - [`num_of_i_pv6routes`](AsnWhoisLookupResponseBuilder::num_of_i_pv6routes)
    /// - [`whois_host`](AsnWhoisLookupResponseBuilder::whois_host)
    /// - [`r#type`](AsnWhoisLookupResponseBuilder::r#type)
    /// - [`route_objects`](AsnWhoisLookupResponseBuilder::route_objects)
    /// - [`whois_response`](AsnWhoisLookupResponseBuilder::whois_response)
    /// - [`downstreams`](AsnWhoisLookupResponseBuilder::downstreams)
    /// - [`parsed_whois_response`](AsnWhoisLookupResponseBuilder::parsed_whois_response)
    /// - [`upstreams`](AsnWhoisLookupResponseBuilder::upstreams)
    /// - [`peers`](AsnWhoisLookupResponseBuilder::peers)
    /// - [`contacts`](AsnWhoisLookupResponseBuilder::contacts)
    /// - [`legacy_routes`](AsnWhoisLookupResponseBuilder::legacy_routes)
    pub fn build(self) -> Result<AsnWhoisLookupResponse, BuildError> {
        Ok(AsnWhoisLookupResponse {
            as_number: self
                .as_number
                .ok_or_else(|| BuildError::missing_field("as_number"))?,
            as_name: self
                .as_name
                .ok_or_else(|| BuildError::missing_field("as_name"))?,
            org_name: self
                .org_name
                .ok_or_else(|| BuildError::missing_field("org_name"))?,
            description: self.description,
            org_handle: self
                .org_handle
                .ok_or_else(|| BuildError::missing_field("org_handle"))?,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            domain: self.domain,
            website: self.website,
            allocation_status: self
                .allocation_status
                .ok_or_else(|| BuildError::missing_field("allocation_status"))?,
            num_of_i_pv4routes: self
                .num_of_i_pv4routes
                .ok_or_else(|| BuildError::missing_field("num_of_i_pv4routes"))?,
            num_of_i_pv6routes: self
                .num_of_i_pv6routes
                .ok_or_else(|| BuildError::missing_field("num_of_i_pv6routes"))?,
            whois_host: self
                .whois_host
                .ok_or_else(|| BuildError::missing_field("whois_host"))?,
            date_allocated: self.date_allocated,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            route_objects: self
                .route_objects
                .ok_or_else(|| BuildError::missing_field("route_objects"))?,
            whois_response: self
                .whois_response
                .ok_or_else(|| BuildError::missing_field("whois_response"))?,
            downstreams: self
                .downstreams
                .ok_or_else(|| BuildError::missing_field("downstreams"))?,
            parsed_whois_response: self
                .parsed_whois_response
                .ok_or_else(|| BuildError::missing_field("parsed_whois_response"))?,
            upstreams: self
                .upstreams
                .ok_or_else(|| BuildError::missing_field("upstreams"))?,
            peers: self
                .peers
                .ok_or_else(|| BuildError::missing_field("peers"))?,
            contacts: self
                .contacts
                .ok_or_else(|| BuildError::missing_field("contacts"))?,
            legacy_routes: self
                .legacy_routes
                .ok_or_else(|| BuildError::missing_field("legacy_routes"))?,
            whois_raw_response: self.whois_raw_response,
            r_whois_raw_response: self.r_whois_raw_response,
            query_time: self.query_time,
        })
    }
}
