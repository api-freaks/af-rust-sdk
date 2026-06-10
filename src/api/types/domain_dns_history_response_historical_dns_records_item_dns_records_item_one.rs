pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOne {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#type: i64,
    #[serde(rename = "dnsType")]
    pub dns_type: DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOneDnsType,
    #[serde(default)]
    pub ttl: i64,
    #[serde(rename = "rawText")]
    #[serde(default)]
    pub raw_text: String,
    #[serde(rename = "rRsetType")]
    #[serde(default)]
    pub r_rset_type: i64,
    /// Host to which this domain points
    #[serde(default)]
    pub target: String,
}

impl DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOne {
    pub fn builder() -> DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOneBuilder {
        <DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOneBuilder {
    name: Option<String>,
    r#type: Option<i64>,
    dns_type: Option<DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOneDnsType>,
    ttl: Option<i64>,
    raw_text: Option<String>,
    r_rset_type: Option<i64>,
    target: Option<String>,
}

impl DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOneBuilder {
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
        value: DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOneDnsType,
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

    /// Consumes the builder and constructs a [`DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOneBuilder::name)
    /// - [`r#type`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOneBuilder::r#type)
    /// - [`dns_type`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOneBuilder::dns_type)
    /// - [`ttl`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOneBuilder::ttl)
    /// - [`raw_text`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOneBuilder::raw_text)
    /// - [`r_rset_type`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOneBuilder::r_rset_type)
    /// - [`target`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOneBuilder::target)
    pub fn build(
        self,
    ) -> Result<DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOne, BuildError> {
        Ok(
            DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemOne {
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
            },
        )
    }
}
