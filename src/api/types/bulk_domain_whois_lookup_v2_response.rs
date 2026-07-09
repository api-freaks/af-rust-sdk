pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkDomainWhoisLookupV2Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_whois_response: Option<Vec<BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItem>>,
}

impl BulkDomainWhoisLookupV2Response {
    pub fn builder() -> BulkDomainWhoisLookupV2ResponseBuilder {
        <BulkDomainWhoisLookupV2ResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDomainWhoisLookupV2ResponseBuilder {
    bulk_whois_response: Option<Vec<BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItem>>,
}

impl BulkDomainWhoisLookupV2ResponseBuilder {
    pub fn bulk_whois_response(
        mut self,
        value: Vec<BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItem>,
    ) -> Self {
        self.bulk_whois_response = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkDomainWhoisLookupV2Response`].
    pub fn build(self) -> Result<BulkDomainWhoisLookupV2Response, BuildError> {
        Ok(BulkDomainWhoisLookupV2Response {
            bulk_whois_response: self.bulk_whois_response,
        })
    }
}
