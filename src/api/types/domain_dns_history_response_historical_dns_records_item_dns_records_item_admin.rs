pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdmin {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#type: i64,
    #[serde(rename = "dnsType")]
    pub dns_type: DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdminDnsType,
    #[serde(default)]
    pub ttl: i64,
    #[serde(rename = "rawText")]
    #[serde(default)]
    pub raw_text: String,
    #[serde(rename = "rRsetType")]
    #[serde(default)]
    pub r_rset_type: i64,
    #[serde(default)]
    pub admin: String,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub expire: i64,
    #[serde(default)]
    pub minimum: i64,
    #[serde(default)]
    pub refresh: i64,
    #[serde(default)]
    pub retry: i64,
    #[serde(default)]
    pub serial: i64,
}

impl DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdmin {
    pub fn builder() -> DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdminBuilder {
        <DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdminBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdminBuilder {
    name: Option<String>,
    r#type: Option<i64>,
    dns_type: Option<DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdminDnsType>,
    ttl: Option<i64>,
    raw_text: Option<String>,
    r_rset_type: Option<i64>,
    admin: Option<String>,
    host: Option<String>,
    expire: Option<i64>,
    minimum: Option<i64>,
    refresh: Option<i64>,
    retry: Option<i64>,
    serial: Option<i64>,
}

impl DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdminBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: i64) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn dns_type(
        mut self,
        value: DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdminDnsType,
    ) -> Self {
        self.dns_type = Some(value);
        self
    }

    pub fn ttl(mut self, value: i64) -> Self {
        self.ttl = Some(value);
        self
    }

    pub fn raw_text(mut self, value: impl Into<String>) -> Self {
        self.raw_text = Some(value.into());
        self
    }

    pub fn r_rset_type(mut self, value: i64) -> Self {
        self.r_rset_type = Some(value);
        self
    }

    pub fn admin(mut self, value: impl Into<String>) -> Self {
        self.admin = Some(value.into());
        self
    }

    pub fn host(mut self, value: impl Into<String>) -> Self {
        self.host = Some(value.into());
        self
    }

    pub fn expire(mut self, value: i64) -> Self {
        self.expire = Some(value);
        self
    }

    pub fn minimum(mut self, value: i64) -> Self {
        self.minimum = Some(value);
        self
    }

    pub fn refresh(mut self, value: i64) -> Self {
        self.refresh = Some(value);
        self
    }

    pub fn retry(mut self, value: i64) -> Self {
        self.retry = Some(value);
        self
    }

    pub fn serial(mut self, value: i64) -> Self {
        self.serial = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdmin`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdminBuilder::name)
    /// - [`r#type`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdminBuilder::r#type)
    /// - [`dns_type`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdminBuilder::dns_type)
    /// - [`ttl`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdminBuilder::ttl)
    /// - [`raw_text`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdminBuilder::raw_text)
    /// - [`r_rset_type`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdminBuilder::r_rset_type)
    /// - [`admin`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdminBuilder::admin)
    /// - [`host`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdminBuilder::host)
    /// - [`expire`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdminBuilder::expire)
    /// - [`minimum`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdminBuilder::minimum)
    /// - [`refresh`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdminBuilder::refresh)
    /// - [`retry`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdminBuilder::retry)
    /// - [`serial`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdminBuilder::serial)
    pub fn build(
        self,
    ) -> Result<DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdmin, BuildError>
    {
        Ok(
            DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemAdmin {
                name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
                r#type: self
                    .r#type
                    .ok_or_else(|| BuildError::missing_field("r#type"))?,
                dns_type: self
                    .dns_type
                    .ok_or_else(|| BuildError::missing_field("dns_type"))?,
                ttl: self.ttl.ok_or_else(|| BuildError::missing_field("ttl"))?,
                raw_text: self
                    .raw_text
                    .ok_or_else(|| BuildError::missing_field("raw_text"))?,
                r_rset_type: self
                    .r_rset_type
                    .ok_or_else(|| BuildError::missing_field("r_rset_type"))?,
                admin: self
                    .admin
                    .ok_or_else(|| BuildError::missing_field("admin"))?,
                host: self.host.ok_or_else(|| BuildError::missing_field("host"))?,
                expire: self
                    .expire
                    .ok_or_else(|| BuildError::missing_field("expire"))?,
                minimum: self
                    .minimum
                    .ok_or_else(|| BuildError::missing_field("minimum"))?,
                refresh: self
                    .refresh
                    .ok_or_else(|| BuildError::missing_field("refresh"))?,
                retry: self
                    .retry
                    .ok_or_else(|| BuildError::missing_field("retry"))?,
                serial: self
                    .serial
                    .ok_or_else(|| BuildError::missing_field("serial"))?,
            },
        )
    }
}
