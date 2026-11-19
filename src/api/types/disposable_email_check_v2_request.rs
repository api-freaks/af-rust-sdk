pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisposableEmailCheckV2Request {
    /// The email address to check.
    #[serde(default)]
    pub email: String,
    /// Optional IP address to look up for location and security data, such as the IP the signup came from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(skip)]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response.
    #[serde(skip)]
    pub format: Option<DisposableEmailCheckV2RequestFormat>,
}

impl DisposableEmailCheckV2Request {
    pub fn builder() -> DisposableEmailCheckV2RequestBuilder {
        <DisposableEmailCheckV2RequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisposableEmailCheckV2RequestBuilder {
    email: Option<String>,
    ip: Option<String>,
    api_key: Option<String>,
    format: Option<DisposableEmailCheckV2RequestFormat>,
}

impl DisposableEmailCheckV2RequestBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
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

    pub fn format(mut self, value: DisposableEmailCheckV2RequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DisposableEmailCheckV2Request`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](DisposableEmailCheckV2RequestBuilder::email)
    /// - [`api_key`](DisposableEmailCheckV2RequestBuilder::api_key)
    pub fn build(self) -> Result<DisposableEmailCheckV2Request, BuildError> {
        Ok(DisposableEmailCheckV2Request {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            ip: self.ip,
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
        })
    }
}
