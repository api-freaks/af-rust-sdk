pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AsnWhoisLookupResponseParsedWhoisResponseAutNumsItem {
    #[serde(default)]
    pub aut_num: String,
    #[serde(default)]
    pub as_handle: String,
    #[serde(default)]
    pub as_name: String,
    #[serde(default)]
    pub tech_contacts: Vec<String>,
    #[serde(default)]
    pub abuse_contacts: Vec<String>,
    #[serde(default)]
    pub date_created: NaiveDate,
    #[serde(default)]
    pub date_updated: NaiveDate,
    #[serde(default)]
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_via: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp_import: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_via: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp_export: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp_default: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sponsoring_organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_contacts: Option<Vec<String>>,
}

impl AsnWhoisLookupResponseParsedWhoisResponseAutNumsItem {
    pub fn builder() -> AsnWhoisLookupResponseParsedWhoisResponseAutNumsItemBuilder {
        <AsnWhoisLookupResponseParsedWhoisResponseAutNumsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AsnWhoisLookupResponseParsedWhoisResponseAutNumsItemBuilder {
    aut_num: Option<String>,
    as_handle: Option<String>,
    as_name: Option<String>,
    tech_contacts: Option<Vec<String>>,
    abuse_contacts: Option<Vec<String>>,
    date_created: Option<NaiveDate>,
    date_updated: Option<NaiveDate>,
    source: Option<String>,
    description: Option<String>,
    country: Option<String>,
    status: Option<Vec<String>>,
    member_of: Option<Vec<String>>,
    import_via: Option<Vec<String>>,
    import: Option<Vec<String>>,
    mp_import: Option<Vec<String>>,
    export_via: Option<Vec<String>>,
    export: Option<Vec<String>>,
    mp_export: Option<Vec<String>>,
    default: Option<Vec<String>>,
    mp_default: Option<Vec<String>>,
    organization: Option<String>,
    sponsoring_organization: Option<String>,
    admin_contacts: Option<Vec<String>>,
}

impl AsnWhoisLookupResponseParsedWhoisResponseAutNumsItemBuilder {
    pub fn aut_num(mut self, value: impl Into<String>) -> Self {
        self.aut_num = Some(value.into());
        self
    }

    pub fn as_handle(mut self, value: impl Into<String>) -> Self {
        self.as_handle = Some(value.into());
        self
    }

    pub fn as_name(mut self, value: impl Into<String>) -> Self {
        self.as_name = Some(value.into());
        self
    }

    pub fn tech_contacts(mut self, value: Vec<String>) -> Self {
        self.tech_contacts = Some(value);
        self
    }

    pub fn abuse_contacts(mut self, value: Vec<String>) -> Self {
        self.abuse_contacts = Some(value);
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

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn status(mut self, value: Vec<String>) -> Self {
        self.status = Some(value);
        self
    }

    pub fn member_of(mut self, value: Vec<String>) -> Self {
        self.member_of = Some(value);
        self
    }

    pub fn import_via(mut self, value: Vec<String>) -> Self {
        self.import_via = Some(value);
        self
    }

    pub fn import(mut self, value: Vec<String>) -> Self {
        self.import = Some(value);
        self
    }

    pub fn mp_import(mut self, value: Vec<String>) -> Self {
        self.mp_import = Some(value);
        self
    }

    pub fn export_via(mut self, value: Vec<String>) -> Self {
        self.export_via = Some(value);
        self
    }

    pub fn export(mut self, value: Vec<String>) -> Self {
        self.export = Some(value);
        self
    }

    pub fn mp_export(mut self, value: Vec<String>) -> Self {
        self.mp_export = Some(value);
        self
    }

    pub fn default(mut self, value: Vec<String>) -> Self {
        self.default = Some(value);
        self
    }

    pub fn mp_default(mut self, value: Vec<String>) -> Self {
        self.mp_default = Some(value);
        self
    }

    pub fn organization(mut self, value: impl Into<String>) -> Self {
        self.organization = Some(value.into());
        self
    }

    pub fn sponsoring_organization(mut self, value: impl Into<String>) -> Self {
        self.sponsoring_organization = Some(value.into());
        self
    }

    pub fn admin_contacts(mut self, value: Vec<String>) -> Self {
        self.admin_contacts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AsnWhoisLookupResponseParsedWhoisResponseAutNumsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`aut_num`](AsnWhoisLookupResponseParsedWhoisResponseAutNumsItemBuilder::aut_num)
    /// - [`as_handle`](AsnWhoisLookupResponseParsedWhoisResponseAutNumsItemBuilder::as_handle)
    /// - [`as_name`](AsnWhoisLookupResponseParsedWhoisResponseAutNumsItemBuilder::as_name)
    /// - [`tech_contacts`](AsnWhoisLookupResponseParsedWhoisResponseAutNumsItemBuilder::tech_contacts)
    /// - [`abuse_contacts`](AsnWhoisLookupResponseParsedWhoisResponseAutNumsItemBuilder::abuse_contacts)
    /// - [`date_created`](AsnWhoisLookupResponseParsedWhoisResponseAutNumsItemBuilder::date_created)
    /// - [`date_updated`](AsnWhoisLookupResponseParsedWhoisResponseAutNumsItemBuilder::date_updated)
    /// - [`source`](AsnWhoisLookupResponseParsedWhoisResponseAutNumsItemBuilder::source)
    pub fn build(self) -> Result<AsnWhoisLookupResponseParsedWhoisResponseAutNumsItem, BuildError> {
        Ok(AsnWhoisLookupResponseParsedWhoisResponseAutNumsItem {
            aut_num: self
                .aut_num
                .ok_or_else(|| BuildError::missing_field("aut_num"))?,
            as_handle: self
                .as_handle
                .ok_or_else(|| BuildError::missing_field("as_handle"))?,
            as_name: self
                .as_name
                .ok_or_else(|| BuildError::missing_field("as_name"))?,
            tech_contacts: self
                .tech_contacts
                .ok_or_else(|| BuildError::missing_field("tech_contacts"))?,
            abuse_contacts: self
                .abuse_contacts
                .ok_or_else(|| BuildError::missing_field("abuse_contacts"))?,
            date_created: self
                .date_created
                .ok_or_else(|| BuildError::missing_field("date_created"))?,
            date_updated: self
                .date_updated
                .ok_or_else(|| BuildError::missing_field("date_updated"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            description: self.description,
            country: self.country,
            status: self.status,
            member_of: self.member_of,
            import_via: self.import_via,
            import: self.import,
            mp_import: self.mp_import,
            export_via: self.export_via,
            export: self.export,
            mp_export: self.mp_export,
            default: self.default,
            mp_default: self.mp_default,
            organization: self.organization,
            sponsoring_organization: self.sponsoring_organization,
            admin_contacts: self.admin_contacts,
        })
    }
}
