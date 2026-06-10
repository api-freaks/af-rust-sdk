pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PhoneValidateRequest {
    /// Phone number to validate. Accepts international format (+14155552671), local format (4155552671) with region, or IDD format (0014155552671) with dialer_region.
    #[serde(default)]
    pub number: String,
    /// Two-letter ISO country code (e.g., US, GB). Required when number is in local format without + prefix. Cannot be used together with dialer_region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Two-letter ISO country code indicating the country the number is being dialed from. Required when number uses IDD exit code. Cannot be used together with region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialer_region: Option<String>,
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
    /// Specifies the desired format for the API response. Choose 'json' for a JSON object. If not provided, the API defaults to JSON format.
    #[serde(skip_serializing)]
    pub format: Option<PhoneValidateRequestFormat>,
}

impl PhoneValidateRequest {
    pub fn builder() -> PhoneValidateRequestBuilder {
        <PhoneValidateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PhoneValidateRequestBuilder {
    number: Option<String>,
    region: Option<String>,
    dialer_region: Option<String>,
    api_key: Option<String>,
    format: Option<PhoneValidateRequestFormat>,
}

impl PhoneValidateRequestBuilder {
    pub fn number(mut self, value: impl Into<String>) -> Self {
        self.number = Some(value.into());
        self
    }

    pub fn region(mut self, value: impl Into<String>) -> Self {
        self.region = Some(value.into());
        self
    }

    pub fn dialer_region(mut self, value: impl Into<String>) -> Self {
        self.dialer_region = Some(value.into());
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: PhoneValidateRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PhoneValidateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`number`](PhoneValidateRequestBuilder::number)
    /// - [`api_key`](PhoneValidateRequestBuilder::api_key)
    pub fn build(self) -> Result<PhoneValidateRequest, BuildError> {
        Ok(PhoneValidateRequest {
            number: self
                .number
                .ok_or_else(|| BuildError::missing_field("number"))?,
            region: self.region,
            dialer_region: self.dialer_region,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
        })
    }
}
