pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkDomainDnsLookupResponse {
    #[serde(default)]
    pub bulk_dns_info: Vec<BulkDomainDnsLookupResponseBulkDnsInfoItem>,
}

impl BulkDomainDnsLookupResponse {
    pub fn builder() -> BulkDomainDnsLookupResponseBuilder {
        <BulkDomainDnsLookupResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDomainDnsLookupResponseBuilder {
    bulk_dns_info: Option<Vec<BulkDomainDnsLookupResponseBulkDnsInfoItem>>,
}

impl BulkDomainDnsLookupResponseBuilder {
    pub fn bulk_dns_info(mut self, value: Vec<BulkDomainDnsLookupResponseBulkDnsInfoItem>) -> Self {
        self.bulk_dns_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkDomainDnsLookupResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bulk_dns_info`](BulkDomainDnsLookupResponseBuilder::bulk_dns_info)
    pub fn build(self) -> Result<BulkDomainDnsLookupResponse, BuildError> {
        Ok(BulkDomainDnsLookupResponse {
            bulk_dns_info: self
                .bulk_dns_info
                .ok_or_else(|| BuildError::missing_field("bulk_dns_info"))?,
        })
    }
}
