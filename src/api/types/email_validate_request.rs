pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmailValidateRequest {
    /// Email address to validate
    #[serde(default)]
    pub email: String,
    /// Name of the email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// IP address of the email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response
    #[serde(skip_serializing)]
    pub format: Option<EmailValidateRequestFormat>,
}

impl EmailValidateRequest {
    pub fn builder() -> EmailValidateRequestBuilder {
        <EmailValidateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmailValidateRequestBuilder {
    email: Option<String>,
    name: Option<String>,
    ip: Option<String>,
    api_key: Option<String>,
    format: Option<EmailValidateRequestFormat>,
}

impl EmailValidateRequestBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: EmailValidateRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EmailValidateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](EmailValidateRequestBuilder::email)
    /// - [`api_key`](EmailValidateRequestBuilder::api_key)
    pub fn build(self) -> Result<EmailValidateRequest, BuildError> {
        Ok(EmailValidateRequest {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            name: self.name,
            ip: self.ip,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
        })
    }
}
