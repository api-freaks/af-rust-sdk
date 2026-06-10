pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BulkEmailValidateResponse {
    /// Array of SingleEmailValidationResponse objects for bulk validation
    #[serde(rename = "emailValidationResponses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_validation_responses:
        Option<Vec<BulkEmailValidateResponseEmailValidationResponsesItem>>,
}

impl BulkEmailValidateResponse {
    pub fn builder() -> BulkEmailValidateResponseBuilder {
        <BulkEmailValidateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkEmailValidateResponseBuilder {
    email_validation_responses: Option<Vec<BulkEmailValidateResponseEmailValidationResponsesItem>>,
}

impl BulkEmailValidateResponseBuilder {
    pub fn email_validation_responses(
        mut self,
        value: Vec<BulkEmailValidateResponseEmailValidationResponsesItem>,
    ) -> Self {
        self.email_validation_responses = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkEmailValidateResponse`].
    pub fn build(self) -> Result<BulkEmailValidateResponse, BuildError> {
        Ok(BulkEmailValidateResponse {
            email_validation_responses: self.email_validation_responses,
        })
    }
}
