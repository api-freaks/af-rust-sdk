pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAddress {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#type: i64,
    #[serde(rename = "dnsType")]
    pub dns_type: BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAddressDnsType,
    #[serde(default)]
    pub ttl: i64,
    #[serde(rename = "rawText")]
    #[serde(default)]
    pub raw_text: String,
    #[serde(rename = "rRsetType")]
    #[serde(default)]
    pub r_rset_type: i64,
    /// IPv4 or IPv6 address
    #[serde(default)]
    pub address: String,
}

impl BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAddress {
    pub fn builder() -> BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAddressBuilder {
        <BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAddressBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAddressBuilder {
    name: Option<String>,
    r#type: Option<i64>,
    dns_type: Option<BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAddressDnsType>,
    ttl: Option<i64>,
    raw_text: Option<String>,
    r_rset_type: Option<i64>,
    address: Option<String>,
}

impl BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAddressBuilder {
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
        value: BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAddressDnsType,
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

    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAddress`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAddressBuilder::name)
    /// - [`r#type`](BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAddressBuilder::r#type)
    /// - [`dns_type`](BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAddressBuilder::dns_type)
    /// - [`ttl`](BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAddressBuilder::ttl)
    /// - [`raw_text`](BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAddressBuilder::raw_text)
    /// - [`r_rset_type`](BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAddressBuilder::r_rset_type)
    /// - [`address`](BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAddressBuilder::address)
    pub fn build(
        self,
    ) -> Result<BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAddress, BuildError> {
        Ok(
            BulkDomainDnsLookupResponseBulkDnsInfoItemDnsRecordsItemAddress {
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
                address: self
                    .address
                    .ok_or_else(|| BuildError::missing_field("address"))?,
            },
        )
    }
}
