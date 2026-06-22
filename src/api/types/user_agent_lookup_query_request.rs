pub use crate::prelude::*;

/// Query parameters for user_agent_lookup
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserAgentLookupQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Format of the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<UserAgentLookupRequestFormat>,
    /// The User-Agent string to parse. Sent as the `User-Agent` HTTP header.
    #[serde(skip_serializing)]
    #[serde(default)]
    pub user_agent: String,
}

impl UserAgentLookupQueryRequest {
    pub fn builder() -> UserAgentLookupQueryRequestBuilder {
        <UserAgentLookupQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserAgentLookupQueryRequestBuilder {
    api_key: Option<String>,
    format: Option<UserAgentLookupRequestFormat>,
    user_agent: Option<String>,
}

impl UserAgentLookupQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn format(mut self, value: UserAgentLookupRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn user_agent(mut self, value: impl Into<String>) -> Self {
        self.user_agent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserAgentLookupQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](UserAgentLookupQueryRequestBuilder::api_key)
    /// - [`user_agent`](UserAgentLookupQueryRequestBuilder::user_agent)
    pub fn build(self) -> Result<UserAgentLookupQueryRequest, BuildError> {
        Ok(UserAgentLookupQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            format: self.format,
            user_agent: self
                .user_agent
                .ok_or_else(|| BuildError::missing_field("user_agent"))?,
        })
    }
}
