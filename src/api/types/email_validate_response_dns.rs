pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmailValidateResponseDns {
    #[serde(rename = "mxRecords")]
    #[serde(default)]
    pub mx_records: Vec<String>,
    /// Collection of A (Address) records for the domain.
    #[serde(rename = "aRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_records: Option<Vec<String>>,
}

impl EmailValidateResponseDns {
    pub fn builder() -> EmailValidateResponseDnsBuilder {
        <EmailValidateResponseDnsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmailValidateResponseDnsBuilder {
    mx_records: Option<Vec<String>>,
    a_records: Option<Vec<String>>,
}

impl EmailValidateResponseDnsBuilder {
    pub fn mx_records(mut self, value: Vec<String>) -> Self {
        self.mx_records = Some(value);
        self
    }

    pub fn a_records(mut self, value: Vec<String>) -> Self {
        self.a_records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EmailValidateResponseDns`].
    /// This method will fail if any of the following fields are not set:
    /// - [`mx_records`](EmailValidateResponseDnsBuilder::mx_records)
    pub fn build(self) -> Result<EmailValidateResponseDns, BuildError> {
        Ok(EmailValidateResponseDns {
            mx_records: self
                .mx_records
                .ok_or_else(|| BuildError::missing_field("mx_records"))?,
            a_records: self.a_records,
        })
    }
}
