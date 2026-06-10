pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStrings {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#type: i64,
    #[serde(rename = "dnsType")]
    pub dns_type: BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStringsDnsType,
    #[serde(default)]
    pub ttl: i64,
    #[serde(rename = "rawText")]
    #[serde(default)]
    pub raw_text: String,
    #[serde(rename = "rRsetType")]
    #[serde(default)]
    pub r_rset_type: i64,
    #[serde(default)]
    pub strings: Vec<String>,
}

impl BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStrings {
    pub fn builder() -> BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStringsBuilder {
        <BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStringsBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStringsBuilder {
    name: Option<String>,
    r#type: Option<i64>,
    dns_type: Option<BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStringsDnsType>,
    ttl: Option<i64>,
    raw_text: Option<String>,
    r_rset_type: Option<i64>,
    strings: Option<Vec<String>>,
}

impl BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStringsBuilder {
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
        value: BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStringsDnsType,
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

    pub fn strings(mut self, value: Vec<String>) -> Self {
        self.strings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStrings`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStringsBuilder::name)
    /// - [`r#type`](BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStringsBuilder::r#type)
    /// - [`dns_type`](BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStringsBuilder::dns_type)
    /// - [`ttl`](BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStringsBuilder::ttl)
    /// - [`raw_text`](BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStringsBuilder::raw_text)
    /// - [`r_rset_type`](BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStringsBuilder::r_rset_type)
    /// - [`strings`](BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStringsBuilder::strings)
    pub fn build(
        self,
    ) -> Result<BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStrings, BuildError> {
        Ok(
            BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemStrings {
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
                strings: self
                    .strings
                    .ok_or_else(|| BuildError::missing_field("strings"))?,
            },
        )
    }
}
