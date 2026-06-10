pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemSingleName {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#type: i64,
    #[serde(rename = "dnsType")]
    pub dns_type: DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemSingleNameDnsType,
    #[serde(default)]
    pub ttl: i64,
    #[serde(rename = "rawText")]
    #[serde(default)]
    pub raw_text: String,
    #[serde(rename = "rRsetType")]
    #[serde(default)]
    pub r_rset_type: i64,
    /// Name server for the domain
    #[serde(rename = "singleName")]
    #[serde(default)]
    pub single_name: String,
}

impl DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemSingleName {
    pub fn builder(
    ) -> DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemSingleNameBuilder {
        <DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemSingleNameBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemSingleNameBuilder {
    name: Option<String>,
    r#type: Option<i64>,
    dns_type:
        Option<DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemSingleNameDnsType>,
    ttl: Option<i64>,
    raw_text: Option<String>,
    r_rset_type: Option<i64>,
    single_name: Option<String>,
}

impl DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemSingleNameBuilder {
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
        value: DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemSingleNameDnsType,
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

    pub fn single_name(mut self, value: impl Into<String>) -> Self {
        self.single_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemSingleName`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemSingleNameBuilder::name)
    /// - [`r#type`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemSingleNameBuilder::r#type)
    /// - [`dns_type`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemSingleNameBuilder::dns_type)
    /// - [`ttl`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemSingleNameBuilder::ttl)
    /// - [`raw_text`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemSingleNameBuilder::raw_text)
    /// - [`r_rset_type`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemSingleNameBuilder::r_rset_type)
    /// - [`single_name`](DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemSingleNameBuilder::single_name)
    pub fn build(
        self,
    ) -> Result<DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemSingleName, BuildError>
    {
        Ok(
            DomainDnsHistoryResponseHistoricalDnsRecordsItemDnsRecordsItemSingleName {
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
                single_name: self
                    .single_name
                    .ok_or_else(|| BuildError::missing_field("single_name"))?,
            },
        )
    }
}
