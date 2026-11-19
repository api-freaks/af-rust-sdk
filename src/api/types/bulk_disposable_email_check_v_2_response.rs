pub use crate::prelude::*;

/// Wrapper containing the per-email results array.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkDisposableEmailCheckV2Response {
    /// Array of per-email results, one entry per requested email, in the same order as the request.
    #[serde(rename = "disposableEmailResponse")]
    #[serde(default)]
    pub disposable_email_response:
        Vec<BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItem>,
}

impl BulkDisposableEmailCheckV2Response {
    pub fn builder() -> BulkDisposableEmailCheckV2ResponseBuilder {
        <BulkDisposableEmailCheckV2ResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkDisposableEmailCheckV2ResponseBuilder {
    disposable_email_response:
        Option<Vec<BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItem>>,
}

impl BulkDisposableEmailCheckV2ResponseBuilder {
    pub fn disposable_email_response(
        mut self,
        value: Vec<BulkDisposableEmailCheckV2ResponseDisposableEmailResponseItem>,
    ) -> Self {
        self.disposable_email_response = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkDisposableEmailCheckV2Response`].
    /// This method will fail if any of the following fields are not set:
    /// - [`disposable_email_response`](BulkDisposableEmailCheckV2ResponseBuilder::disposable_email_response)
    pub fn build(self) -> Result<BulkDisposableEmailCheckV2Response, BuildError> {
        Ok(BulkDisposableEmailCheckV2Response {
            disposable_email_response: self
                .disposable_email_response
                .ok_or_else(|| BuildError::missing_field("disposable_email_response"))?,
        })
    }
}
