pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DomainDnsLookupResponseDnsRecordsItemStrings {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#type: i64,
    #[serde(rename = "dnsType")]
    pub dns_type: DomainDnsLookupResponseDnsRecordsItemStringsDnsType,
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

impl DomainDnsLookupResponseDnsRecordsItemStrings {
    pub fn builder() -> DomainDnsLookupResponseDnsRecordsItemStringsBuilder {
        <DomainDnsLookupResponseDnsRecordsItemStringsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainDnsLookupResponseDnsRecordsItemStringsBuilder {
    name: Option<String>,
    r#type: Option<i64>,
    dns_type: Option<DomainDnsLookupResponseDnsRecordsItemStringsDnsType>,
    ttl: Option<i64>,
    raw_text: Option<String>,
    r_rset_type: Option<i64>,
    strings: Option<Vec<String>>,
}

impl DomainDnsLookupResponseDnsRecordsItemStringsBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: i64) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn dns_type(mut self, value: DomainDnsLookupResponseDnsRecordsItemStringsDnsType) -> Self {
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

    /// Consumes the builder and constructs a [`DomainDnsLookupResponseDnsRecordsItemStrings`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](DomainDnsLookupResponseDnsRecordsItemStringsBuilder::name)
    /// - [`r#type`](DomainDnsLookupResponseDnsRecordsItemStringsBuilder::r#type)
    /// - [`dns_type`](DomainDnsLookupResponseDnsRecordsItemStringsBuilder::dns_type)
    /// - [`ttl`](DomainDnsLookupResponseDnsRecordsItemStringsBuilder::ttl)
    /// - [`raw_text`](DomainDnsLookupResponseDnsRecordsItemStringsBuilder::raw_text)
    /// - [`r_rset_type`](DomainDnsLookupResponseDnsRecordsItemStringsBuilder::r_rset_type)
    /// - [`strings`](DomainDnsLookupResponseDnsRecordsItemStringsBuilder::strings)
    pub fn build(self) -> Result<DomainDnsLookupResponseDnsRecordsItemStrings, BuildError> {
        Ok(DomainDnsLookupResponseDnsRecordsItemStrings {
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
        })
    }
}
