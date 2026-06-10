pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ContentTooLargeErrorBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

impl ContentTooLargeErrorBody {
    pub fn builder() -> ContentTooLargeErrorBodyBuilder {
        <ContentTooLargeErrorBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContentTooLargeErrorBodyBuilder {
    timestamp: Option<String>,
    status: Option<i64>,
    error: Option<String>,
    message: Option<String>,
    path: Option<String>,
}

impl ContentTooLargeErrorBodyBuilder {
    pub fn timestamp(mut self, value: impl Into<String>) -> Self {
        self.timestamp = Some(value.into());
        self
    }

    pub fn status(mut self, value: i64) -> Self {
        self.status = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ContentTooLargeErrorBody`].
    pub fn build(self) -> Result<ContentTooLargeErrorBody, BuildError> {
        Ok(ContentTooLargeErrorBody {
            timestamp: self.timestamp,
            status: self.status,
            error: self.error,
            message: self.message,
            path: self.path,
        })
    }
}
