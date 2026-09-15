pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmailValidateResponseDns {
    #[serde(rename = "mxRecord")]
    #[serde(default)]
    pub mx_record: Vec<String>,
    /// Collection of A (Address) records for the domain.
    #[serde(rename = "aRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_record: Option<Vec<String>>,
}

impl EmailValidateResponseDns {
    pub fn builder() -> EmailValidateResponseDnsBuilder {
        <EmailValidateResponseDnsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmailValidateResponseDnsBuilder {
    mx_record: Option<Vec<String>>,
    a_record: Option<Vec<String>>,
}

impl EmailValidateResponseDnsBuilder {
    pub fn mx_record(mut self, value: Vec<String>) -> Self {
        self.mx_record = Some(value);
        self
    }

    pub fn a_record(mut self, value: Vec<String>) -> Self {
        self.a_record = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EmailValidateResponseDns`].
    /// This method will fail if any of the following fields are not set:
    /// - [`mx_record`](EmailValidateResponseDnsBuilder::mx_record)
    pub fn build(self) -> Result<EmailValidateResponseDns, BuildError> {
        Ok(EmailValidateResponseDns {
            mx_record: self
                .mx_record
                .ok_or_else(|| BuildError::missing_field("mx_record"))?,
            a_record: self.a_record,
        })
    }
}
