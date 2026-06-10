pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BulkPhoneValidateRequest {
    /// Array of phone number objects. Maximum 100 per request.
    #[serde(default)]
    pub numbers: Vec<BulkPhoneValidateRequestNumbersItem>,
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
    /// Specifies the desired format for the API response. Choose 'json' for a JSON object. If not provided, the API defaults to JSON format.
    #[serde(skip_serializing)]
    pub format: Option<BulkPhoneValidateRequestFormat>,
}

impl BulkPhoneValidateRequest {
    pub fn builder() -> BulkPhoneValidateRequestBuilder {
        <BulkPhoneValidateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BulkPhoneValidateRequestBuilder {
    numbers: Option<Vec<BulkPhoneValidateRequestNumbersItem>>,
    api_key: Option<String>,
    format: Option<BulkPhoneValidateRequestFormat>,
}

impl BulkPhoneValidateRequestBuilder {
    pub fn numbers(mut self, value: Vec<BulkPhoneValidateRequestNumbersItem>) -> Self {
        self.numbers = Some(value);
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: BulkPhoneValidateRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BulkPhoneValidateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`numbers`](BulkPhoneValidateRequestBuilder::numbers)
    /// - [`api_key`](BulkPhoneValidateRequestBuilder::api_key)
    pub fn build(self) -> Result<BulkPhoneValidateRequest, BuildError> {
        Ok(BulkPhoneValidateRequest {
            numbers: self
                .numbers
                .ok_or_else(|| BuildError::missing_field("numbers"))?,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
        })
    }
}
