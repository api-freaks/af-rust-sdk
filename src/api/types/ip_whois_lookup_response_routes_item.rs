pub use crate::prelude::*;

/// BGP route object containing the route prefix, origin AS, and related metadata.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IpWhoisLookupResponseRoutesItem {
    /// BGP route prefix (e.g., 8.8.8.0/24).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    /// Origin AS number for this route.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// Description values for this route object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    /// Country of the organization managing this route.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Pingable IP addresses in this route.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pingable: Option<Vec<String>>,
    /// Handles of pingable contacts for this route.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ping_hdl: Option<Vec<String>>,
    /// Sub-allocation holes (more specific routes) within this route.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holes: Option<Vec<String>>,
    /// Route sets this route is a member of.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of: Option<Vec<String>>,
    /// Injection details for this route.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inject: Option<Vec<String>>,
    /// Aggregation method for this route.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggr_mtd: Option<String>,
    /// Aggregation boundary for this route.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggr_bndry: Option<String>,
    /// Export composition for this route.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_comps: Option<String>,
    /// Route components.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<String>,
    /// Handles of organization objects related to this route.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizations: Option<Vec<String>>,
    /// Remarks for this route object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<Vec<String>>,
    /// Notify email addresses for this route object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify: Option<Vec<String>>,
    /// Lower-level maintainer handles for this route.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mnt_lower: Option<Vec<String>>,
    /// Maintainer handles for this route object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mnt_by: Option<Vec<String>>,
    /// Registration date of this route object (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<NaiveDate>,
    /// Last update date of this route object (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<NaiveDate>,
    /// WHOIS data source for this route object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl IpWhoisLookupResponseRoutesItem {
    pub fn builder() -> IpWhoisLookupResponseRoutesItemBuilder {
        <IpWhoisLookupResponseRoutesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IpWhoisLookupResponseRoutesItemBuilder {
    route: Option<String>,
    origin: Option<String>,
    description: Option<Vec<String>>,
    country: Option<String>,
    pingable: Option<Vec<String>>,
    ping_hdl: Option<Vec<String>>,
    holes: Option<Vec<String>>,
    member_of: Option<Vec<String>>,
    inject: Option<Vec<String>>,
    aggr_mtd: Option<String>,
    aggr_bndry: Option<String>,
    export_comps: Option<String>,
    components: Option<String>,
    organizations: Option<Vec<String>>,
    remarks: Option<Vec<String>>,
    notify: Option<Vec<String>>,
    mnt_lower: Option<Vec<String>>,
    mnt_by: Option<Vec<String>>,
    date_created: Option<NaiveDate>,
    date_updated: Option<NaiveDate>,
    source: Option<String>,
}

impl IpWhoisLookupResponseRoutesItemBuilder {
    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    pub fn origin(mut self, value: impl Into<String>) -> Self {
        self.origin = Some(value.into());
        self
    }

    pub fn description(mut self, value: Vec<String>) -> Self {
        self.description = Some(value);
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn pingable(mut self, value: Vec<String>) -> Self {
        self.pingable = Some(value);
        self
    }

    pub fn ping_hdl(mut self, value: Vec<String>) -> Self {
        self.ping_hdl = Some(value);
        self
    }

    pub fn holes(mut self, value: Vec<String>) -> Self {
        self.holes = Some(value);
        self
    }

    pub fn member_of(mut self, value: Vec<String>) -> Self {
        self.member_of = Some(value);
        self
    }

    pub fn inject(mut self, value: Vec<String>) -> Self {
        self.inject = Some(value);
        self
    }

    pub fn aggr_mtd(mut self, value: impl Into<String>) -> Self {
        self.aggr_mtd = Some(value.into());
        self
    }

    pub fn aggr_bndry(mut self, value: impl Into<String>) -> Self {
        self.aggr_bndry = Some(value.into());
        self
    }

    pub fn export_comps(mut self, value: impl Into<String>) -> Self {
        self.export_comps = Some(value.into());
        self
    }

    pub fn components(mut self, value: impl Into<String>) -> Self {
        self.components = Some(value.into());
        self
    }

    pub fn organizations(mut self, value: Vec<String>) -> Self {
        self.organizations = Some(value);
        self
    }

    pub fn remarks(mut self, value: Vec<String>) -> Self {
        self.remarks = Some(value);
        self
    }

    pub fn notify(mut self, value: Vec<String>) -> Self {
        self.notify = Some(value);
        self
    }

    pub fn mnt_lower(mut self, value: Vec<String>) -> Self {
        self.mnt_lower = Some(value);
        self
    }

    pub fn mnt_by(mut self, value: Vec<String>) -> Self {
        self.mnt_by = Some(value);
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

    /// Consumes the builder and constructs a [`IpWhoisLookupResponseRoutesItem`].
    pub fn build(self) -> Result<IpWhoisLookupResponseRoutesItem, BuildError> {
        Ok(IpWhoisLookupResponseRoutesItem {
            route: self.route,
            origin: self.origin,
            description: self.description,
            country: self.country,
            pingable: self.pingable,
            ping_hdl: self.ping_hdl,
            holes: self.holes,
            member_of: self.member_of,
            inject: self.inject,
            aggr_mtd: self.aggr_mtd,
            aggr_bndry: self.aggr_bndry,
            export_comps: self.export_comps,
            components: self.components,
            organizations: self.organizations,
            remarks: self.remarks,
            notify: self.notify,
            mnt_lower: self.mnt_lower,
            mnt_by: self.mnt_by,
            date_created: self.date_created,
            date_updated: self.date_updated,
            source: self.source,
        })
    }
}
