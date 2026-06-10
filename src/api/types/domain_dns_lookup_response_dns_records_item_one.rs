pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DomainDnsLookupResponseDnsRecordsItemOne {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#type: i64,
    #[serde(rename = "dnsType")]
    pub dns_type: DomainDnsLookupResponseDnsRecordsItemOneDnsType,
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

impl DomainDnsLookupResponseDnsRecordsItemOne {
    pub fn builder() -> DomainDnsLookupResponseDnsRecordsItemOneBuilder {
        <DomainDnsLookupResponseDnsRecordsItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainDnsLookupResponseDnsRecordsItemOneBuilder {
    name: Option<String>,
    r#type: Option<i64>,
    dns_type: Option<DomainDnsLookupResponseDnsRecordsItemOneDnsType>,
    ttl: Option<i64>,
    raw_text: Option<String>,
    r_rset_type: Option<i64>,
    target: Option<String>,
}

impl DomainDnsLookupResponseDnsRecordsItemOneBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: i64) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn dns_type(mut self, value: DomainDnsLookupResponseDnsRecordsItemOneDnsType) -> Self {
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

    /// Consumes the builder and constructs a [`DomainDnsLookupResponseDnsRecordsItemOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](DomainDnsLookupResponseDnsRecordsItemOneBuilder::name)
    /// - [`r#type`](DomainDnsLookupResponseDnsRecordsItemOneBuilder::r#type)
    /// - [`dns_type`](DomainDnsLookupResponseDnsRecordsItemOneBuilder::dns_type)
    /// - [`ttl`](DomainDnsLookupResponseDnsRecordsItemOneBuilder::ttl)
    /// - [`raw_text`](DomainDnsLookupResponseDnsRecordsItemOneBuilder::raw_text)
    /// - [`r_rset_type`](DomainDnsLookupResponseDnsRecordsItemOneBuilder::r_rset_type)
    /// - [`target`](DomainDnsLookupResponseDnsRecordsItemOneBuilder::target)
    pub fn build(self) -> Result<DomainDnsLookupResponseDnsRecordsItemOne, BuildError> {
        Ok(DomainDnsLookupResponseDnsRecordsItemOne {
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
        })
    }
}
