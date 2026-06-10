pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkDomainWhoisLookupResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_whois_response: Option<Vec<BulkDomainWhoisLookupResponseBulkWhoisResponseItem>>,
}

impl BulkDomainWhoisLookupResponse {
    pub fn builder() -> BulkDomainWhoisLookupResponseBuilder {
        <BulkDomainWhoisLookupResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDomainWhoisLookupResponseBuilder {
    bulk_whois_response: Option<Vec<BulkDomainWhoisLookupResponseBulkWhoisResponseItem>>,
}

impl BulkDomainWhoisLookupResponseBuilder {
    pub fn bulk_whois_response(
        mut self,
        value: Vec<BulkDomainWhoisLookupResponseBulkWhoisResponseItem>,
    ) -> Self {
        self.bulk_whois_response = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkDomainWhoisLookupResponse`].
    pub fn build(self) -> Result<BulkDomainWhoisLookupResponse, BuildError> {
        Ok(BulkDomainWhoisLookupResponse {
            bulk_whois_response: self.bulk_whois_response,
        })
    }
}
