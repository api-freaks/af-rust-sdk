pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatValidateResponseValidation {
    #[serde(default)]
    pub is_valid: bool,
    /// Returned when the upstream authority provides a consultation reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consultation_number: Option<String>,
    #[serde(default)]
    pub consultation_authority: String,
}

impl VatValidateResponseValidation {
    pub fn builder() -> VatValidateResponseValidationBuilder {
        <VatValidateResponseValidationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatValidateResponseValidationBuilder {
    is_valid: Option<bool>,
    consultation_number: Option<String>,
    consultation_authority: Option<String>,
}

impl VatValidateResponseValidationBuilder {
    pub fn is_valid(mut self, value: bool) -> Self {
        self.is_valid = Some(value);
        self
    }

    pub fn consultation_number(mut self, value: impl Into<String>) -> Self {
        self.consultation_number = Some(value.into());
        self
    }

    pub fn consultation_authority(mut self, value: impl Into<String>) -> Self {
        self.consultation_authority = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VatValidateResponseValidation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_valid`](VatValidateResponseValidationBuilder::is_valid)
    /// - [`consultation_authority`](VatValidateResponseValidationBuilder::consultation_authority)
    pub fn build(self) -> Result<VatValidateResponseValidation, BuildError> {
        Ok(VatValidateResponseValidation {
            is_valid: self
                .is_valid
                .ok_or_else(|| BuildError::missing_field("is_valid"))?,
            consultation_number: self.consultation_number,
            consultation_authority: self
                .consultation_authority
                .ok_or_else(|| BuildError::missing_field("consultation_authority"))?,
        })
    }
}
