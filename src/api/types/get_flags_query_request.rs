pub use crate::prelude::*;

/// Query parameters for get_flags
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetFlagsQueryRequest {
    /// Your API key
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Country code in ISO 3166-1 alpha-2 format.
    #[serde(default)]
    pub name: String,
    /// Flag shape. One of: `'flat'` or `'round'`.
    pub shape: GetFlagsRequestShape,
    /// Flag format. Applicable only for PNG or WEBP formats. Default is png.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<GetFlagsRequestFormat>,
    /// Flag size in pixels. Valid options: `16px`, `24px`, `32px`, `48px`, `64px`. Applicable only for PNG or WEBP formats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<GetFlagsRequestSize>,
    /// Type of flag. One of: `country` or `organization`.
    pub r#type: GetFlagsRequestType,
}

impl GetFlagsQueryRequest {
    pub fn builder() -> GetFlagsQueryRequestBuilder {
        <GetFlagsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetFlagsQueryRequestBuilder {
    api_key: Option<String>,
    name: Option<String>,
    shape: Option<GetFlagsRequestShape>,
    format: Option<GetFlagsRequestFormat>,
    size: Option<GetFlagsRequestSize>,
    r#type: Option<GetFlagsRequestType>,
}

impl GetFlagsQueryRequestBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn shape(mut self, value: GetFlagsRequestShape) -> Self {
        self.shape = Some(value);
        self
    }

    pub fn format(mut self, value: GetFlagsRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn size(mut self, value: GetFlagsRequestSize) -> Self {
        self.size = Some(value);
        self
    }

    pub fn r#type(mut self, value: GetFlagsRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetFlagsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](GetFlagsQueryRequestBuilder::api_key)
    /// - [`name`](GetFlagsQueryRequestBuilder::name)
    /// - [`shape`](GetFlagsQueryRequestBuilder::shape)
    /// - [`r#type`](GetFlagsQueryRequestBuilder::r#type)
    pub fn build(self) -> Result<GetFlagsQueryRequest, BuildError> {
        Ok(GetFlagsQueryRequest {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            shape: self
                .shape
                .ok_or_else(|| BuildError::missing_field("shape"))?,
            format: self.format,
            size: self.size,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
