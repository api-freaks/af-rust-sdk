pub use crate::prelude::*;

/// Object contains IBAN validation details.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IbanValidateResponseValidation {
    /// Checks if provided IBAN contains only alpha numeric characters
    #[serde(default)]
    pub is_alpha_numeric: bool,
    /// Checks whether given IBAN country supports IBAN standards
    #[serde(default)]
    pub is_iban_supported_country: bool,
    /// Indicates whether IBAN length is according to respective country standard or not.
    #[serde(default)]
    pub is_valid_length: bool,
    /// Indicates whether IBAN structure is valid as per structure pattern for respective country.
    #[serde(default)]
    pub is_valid_structure: bool,
    /// Indicates whether IBAN check digit is valid.
    #[serde(default)]
    pub is_iban_check_digit_valid: bool,
    /// Indicates BBAN checksum is valid or invalid or not supported (unknown) for respective IBAN country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bban: Option<String>,
}

impl IbanValidateResponseValidation {
    pub fn builder() -> IbanValidateResponseValidationBuilder {
        <IbanValidateResponseValidationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IbanValidateResponseValidationBuilder {
    is_alpha_numeric: Option<bool>,
    is_iban_supported_country: Option<bool>,
    is_valid_length: Option<bool>,
    is_valid_structure: Option<bool>,
    is_iban_check_digit_valid: Option<bool>,
    bban: Option<String>,
}

impl IbanValidateResponseValidationBuilder {
    pub fn is_alpha_numeric(mut self, value: bool) -> Self {
        self.is_alpha_numeric = Some(value);
        self
    }

    pub fn is_iban_supported_country(mut self, value: bool) -> Self {
        self.is_iban_supported_country = Some(value);
        self
    }

    pub fn is_valid_length(mut self, value: bool) -> Self {
        self.is_valid_length = Some(value);
        self
    }

    pub fn is_valid_structure(mut self, value: bool) -> Self {
        self.is_valid_structure = Some(value);
        self
    }

    pub fn is_iban_check_digit_valid(mut self, value: bool) -> Self {
        self.is_iban_check_digit_valid = Some(value);
        self
    }

    pub fn bban(mut self, value: impl Into<String>) -> Self {
        self.bban = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IbanValidateResponseValidation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_alpha_numeric`](IbanValidateResponseValidationBuilder::is_alpha_numeric)
    /// - [`is_iban_supported_country`](IbanValidateResponseValidationBuilder::is_iban_supported_country)
    /// - [`is_valid_length`](IbanValidateResponseValidationBuilder::is_valid_length)
    /// - [`is_valid_structure`](IbanValidateResponseValidationBuilder::is_valid_structure)
    /// - [`is_iban_check_digit_valid`](IbanValidateResponseValidationBuilder::is_iban_check_digit_valid)
    pub fn build(self) -> Result<IbanValidateResponseValidation, BuildError> {
        Ok(IbanValidateResponseValidation {
            is_alpha_numeric: self
                .is_alpha_numeric
                .ok_or_else(|| BuildError::missing_field("is_alpha_numeric"))?,
            is_iban_supported_country: self
                .is_iban_supported_country
                .ok_or_else(|| BuildError::missing_field("is_iban_supported_country"))?,
            is_valid_length: self
                .is_valid_length
                .ok_or_else(|| BuildError::missing_field("is_valid_length"))?,
            is_valid_structure: self
                .is_valid_structure
                .ok_or_else(|| BuildError::missing_field("is_valid_structure"))?,
            is_iban_check_digit_valid: self
                .is_iban_check_digit_valid
                .ok_or_else(|| BuildError::missing_field("is_iban_check_digit_valid"))?,
            bban: self.bban,
        })
    }
}
