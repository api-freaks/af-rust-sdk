pub use crate::prelude::*;

/// Wrapper object containing one WHOIS result or error per requested domain, in request order.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkDomainWhoisLookupV2Response {
    /// Array of per-domain results, one entry per requested domain. Each entry is either a full WHOIS result or, if that domain could not be resolved, an error object.
    #[serde(default)]
    pub bulk_whois_response: Vec<BulkDomainWhoisLookupV2ResponseBulkWhoisResponseItem>,
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
    /// This method will fail if any of the following fields are not set:
    /// - [`bulk_whois_response`](BulkDomainWhoisLookupV2ResponseBuilder::bulk_whois_response)
    pub fn build(self) -> Result<BulkDomainWhoisLookupV2Response, BuildError> {
        Ok(BulkDomainWhoisLookupV2Response {
            bulk_whois_response: self
                .bulk_whois_response
                .ok_or_else(|| BuildError::missing_field("bulk_whois_response"))?,
        })
    }
}
