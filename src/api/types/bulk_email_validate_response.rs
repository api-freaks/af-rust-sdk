pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkEmailValidateResponse {
    /// Array of SingleEmailValidationResponse objects for bulk validation
    #[serde(rename = "emailResponse")]
    #[serde(default)]
    pub email_response: Vec<BulkEmailValidateResponseEmailResponseItem>,
}

impl BulkEmailValidateResponse {
    pub fn builder() -> BulkEmailValidateResponseBuilder {
        <BulkEmailValidateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkEmailValidateResponseBuilder {
    email_response: Option<Vec<BulkEmailValidateResponseEmailResponseItem>>,
}

impl BulkEmailValidateResponseBuilder {
    pub fn email_response(
        mut self,
        value: Vec<BulkEmailValidateResponseEmailResponseItem>,
    ) -> Self {
        self.email_response = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkEmailValidateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email_response`](BulkEmailValidateResponseBuilder::email_response)
    pub fn build(self) -> Result<BulkEmailValidateResponse, BuildError> {
        Ok(BulkEmailValidateResponse {
            email_response: self
                .email_response
                .ok_or_else(|| BuildError::missing_field("email_response"))?,
        })
    }
}
