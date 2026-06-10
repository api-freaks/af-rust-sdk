pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriority {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#type: i64,
    #[serde(rename = "dnsType")]
    pub dns_type: DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriorityDnsType,
    #[serde(default)]
    pub ttl: i64,
    #[serde(rename = "rawText")]
    #[serde(default)]
    pub raw_text: String,
    #[serde(rename = "rRsetType")]
    #[serde(default)]
    pub r_rset_type: i64,
    #[serde(default)]
    pub target: String,
    #[serde(default)]
    pub priority: i64,
}

impl DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriority {
    pub fn builder() -> DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriorityBuilder
    {
        <DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriorityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriorityBuilder {
    name: Option<String>,
    r#type: Option<i64>,
    dns_type: Option<DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriorityDnsType>,
    ttl: Option<i64>,
    raw_text: Option<String>,
    r_rset_type: Option<i64>,
    target: Option<String>,
    priority: Option<i64>,
}

impl DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriorityBuilder {
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
        value: DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriorityDnsType,
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

    pub fn target(mut self, value: impl Into<String>) -> Self {
        self.target = Some(value.into());
        self
    }

    pub fn priority(mut self, value: i64) -> Self {
        self.priority = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriority`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriorityBuilder::name)
    /// - [`r#type`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriorityBuilder::r#type)
    /// - [`dns_type`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriorityBuilder::dns_type)
    /// - [`ttl`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriorityBuilder::ttl)
    /// - [`raw_text`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriorityBuilder::raw_text)
    /// - [`r_rset_type`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriorityBuilder::r_rset_type)
    /// - [`target`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriorityBuilder::target)
    /// - [`priority`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriorityBuilder::priority)
    pub fn build(
        self,
    ) -> Result<DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriority, BuildError>
    {
        Ok(
            DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemPriority {
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
                target: self
                    .target
                    .ok_or_else(|| BuildError::missing_field("target"))?,
                priority: self
                    .priority
                    .ok_or_else(|| BuildError::missing_field("priority"))?,
            },
        )
    }
}
