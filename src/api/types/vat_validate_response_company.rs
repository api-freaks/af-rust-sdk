pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatValidateResponseCompany {
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub company_address: String,
}

impl VatValidateResponseCompany {
    pub fn builder() -> VatValidateResponseCompanyBuilder {
        <VatValidateResponseCompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatValidateResponseCompanyBuilder {
    company_name: Option<String>,
    company_address: Option<String>,
}

impl VatValidateResponseCompanyBuilder {
    pub fn company_name(mut self, value: impl Into<String>) -> Self {
        self.company_name = Some(value.into());
        self
    }

    pub fn company_address(mut self, value: impl Into<String>) -> Self {
        self.company_address = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VatValidateResponseCompany`].
    /// This method will fail if any of the following fields are not set:
    /// - [`company_name`](VatValidateResponseCompanyBuilder::company_name)
    /// - [`company_address`](VatValidateResponseCompanyBuilder::company_address)
    pub fn build(self) -> Result<VatValidateResponseCompany, BuildError> {
        Ok(VatValidateResponseCompany {
            company_name: self
                .company_name
                .ok_or_else(|| BuildError::missing_field("company_name"))?,
            company_address: self
                .company_address
                .ok_or_else(|| BuildError::missing_field("company_address"))?,
        })
    }
}
