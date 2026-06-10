pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAddress {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#type: i64,
    #[serde(rename = "dnsType")]
    pub dns_type: DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAddressDnsType,
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

impl DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAddress {
    pub fn builder() -> DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAddressBuilder {
        <DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAddressBuilder {
    name: Option<String>,
    r#type: Option<i64>,
    dns_type: Option<DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAddressDnsType>,
    ttl: Option<i64>,
    raw_text: Option<String>,
    r_rset_type: Option<i64>,
    address: Option<String>,
}

impl DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAddressBuilder {
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
        value: DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAddressDnsType,
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

    /// Consumes the builder and constructs a [`DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAddress`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAddressBuilder::name)
    /// - [`r#type`](DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAddressBuilder::r#type)
    /// - [`dns_type`](DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAddressBuilder::dns_type)
    /// - [`ttl`](DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAddressBuilder::ttl)
    /// - [`raw_text`](DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAddressBuilder::raw_text)
    /// - [`r_rset_type`](DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAddressBuilder::r_rset_type)
    /// - [`address`](DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAddressBuilder::address)
    pub fn build(
        self,
    ) -> Result<DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAddress, BuildError>
    {
        Ok(
            DomainDnsReverseResponseReverseDnsRecordsItemDnsRecordsItemAddress {
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
