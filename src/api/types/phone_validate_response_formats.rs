pub use crate::prelude::*;

/// The number represented in four standardized formats. Only returned for valid numbers.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PhoneValidateResponseFormats {
    /// E.164 format for storage and APIs.
    #[serde(rename = "E164")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e164: Option<String>,
    /// Human-readable international format.
    #[serde(rename = "International")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub international: Option<String>,
    /// Local format as dialed within the country.
    #[serde(rename = "National")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national: Option<String>,
    /// URI format for tel: links.
    #[serde(rename = "RFC3966")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfc3966: Option<String>,
}

impl PhoneValidateResponseFormats {
    pub fn builder() -> PhoneValidateResponseFormatsBuilder {
        <PhoneValidateResponseFormatsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PhoneValidateResponseFormatsBuilder {
    e164: Option<String>,
    international: Option<String>,
    national: Option<String>,
    rfc3966: Option<String>,
}

impl PhoneValidateResponseFormatsBuilder {
    pub fn e164(mut self, value: impl Into<String>) -> Self {
        self.e164 = Some(value.into());
        self
    }

    pub fn international(mut self, value: impl Into<String>) -> Self {
        self.international = Some(value.into());
        self
    }

    pub fn national(mut self, value: impl Into<String>) -> Self {
        self.national = Some(value.into());
        self
    }

    pub fn rfc3966(mut self, value: impl Into<String>) -> Self {
        self.rfc3966 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PhoneValidateResponseFormats`].
    pub fn build(self) -> Result<PhoneValidateResponseFormats, BuildError> {
        Ok(PhoneValidateResponseFormats {
            e164: self.e164,
            international: self.international,
            national: self.national,
            rfc3966: self.rfc3966,
        })
    }
}
